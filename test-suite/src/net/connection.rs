use net;
use net::packet::constants::MAX_PACKET_LENGTH;
use net::packet::io::error::IOError;
use net::packet::io::read::PktRead;
use net::packet::model::Packet;
use sec::aes::AES;
use sec::custom;
use std::collections::VecDeque;
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::error::HarnessError;

pub struct TestConnection {
    endpoint: std::net::SocketAddr,
    buffered_packets: VecDeque<Packet>,
    pub handshake: Handshake,
    stream: TcpStream,
    send_cipher: AES,
    recv_cipher: AES,
}

impl TestConnection {
    pub async fn connect(
        endpoint: std::net::SocketAddr,
        phase: &'static str,
    ) -> Result<Self, HarnessError> {
        let mut stream = TcpStream::connect(endpoint)
            .await
            .map_err(|source| HarnessError::IOError(phase, endpoint.to_string(), source))?;
        let handshake = read_handshake(&mut stream, endpoint, phase).await?;
        let send_cipher = AES::new(&handshake.recv_iv, 83);
        let recv_cipher = AES::new(&handshake.send_iv, 83);
        Ok(Self {
            endpoint,
            buffered_packets: VecDeque::new(),
            handshake,
            stream,
            send_cipher,
            recv_cipher,
        })
    }

    pub async fn read_packet(&mut self, phase: &'static str) -> Result<Packet, HarnessError> {
        if let Some(packet) = self.buffered_packets.pop_front() {
            return Ok(packet);
        }
        let mut header = [0u8; 4];
        self.stream
            .read_exact(&mut header)
            .await
            .map_err(|source| HarnessError::IOError(phase, self.endpoint.to_string(), source))?;
        packet::service::check_header(&self.recv_cipher, &header)?;
        let length = packet::io::read::get_packet_length(&header);
        if !(2..=MAX_PACKET_LENGTH).contains(&length) {
            return Err(HarnessError::PacketIOError(IOError::InvalidPacketLength(
                length,
            )));
        }
        let mut body = vec![0u8; length as usize];
        self.stream
            .read_exact(&mut body)
            .await
            .map_err(|source| HarnessError::IOError(phase, self.endpoint.to_string(), source))?;
        self.recv_cipher.crypt(&mut body);
        custom::decrypt(&mut body);
        Ok(Packet::new(&body))
    }

    pub async fn send_packet(
        &mut self,
        mut packet: Packet,
        phase: &'static str,
    ) -> Result<(), HarnessError> {
        let header = self
            .send_cipher
            .gen_packet_header(packet.bytes.len() as i16);
        custom::encrypt(&mut packet.bytes);
        self.send_cipher.crypt(&mut packet.bytes);
        self.stream
            .write_all(&header)
            .await
            .map_err(|source| HarnessError::IOError(phase, self.endpoint.to_string(), source))?;
        self.stream
            .write_all(&packet.bytes)
            .await
            .map_err(|source| HarnessError::IOError(phase, self.endpoint.to_string(), source))?;
        self.stream
            .flush()
            .await
            .map_err(|source| HarnessError::IOError(phase, self.endpoint.to_string(), source))
    }
}

async fn read_handshake(
    stream: &mut TcpStream,
    endpoint: std::net::SocketAddr,
    phase: &'static str,
) -> Result<Handshake, HarnessError> {
    let mut len_buf = [0u8; 2];
    stream
        .read_exact(&mut len_buf)
        .await
        .map_err(|source| HarnessError::IOError(phase, endpoint.to_string(), source))?;
    let length = i16::from_le_bytes(len_buf);
    if length <= 0 {
        return Err(HarnessError::PacketIOError(IOError::InvalidPacketLength(
            length,
        )));
    }
    let mut payload = vec![0u8; length as usize];
    stream
        .read_exact(&mut payload)
        .await
        .map_err(|source| HarnessError::IOError(phase, endpoint.to_string(), source))?;
    let mut bytes = len_buf.to_vec();
    bytes.extend_from_slice(&payload);
    Handshake::parse(&bytes)
}

#[derive(Debug, Clone)]
pub struct Handshake {
    pub version: i16,
    pub recv_iv: Vec<u8>,
    pub send_iv: Vec<u8>,
    pub locale: u8,
}

impl Handshake {
    pub fn parse(bytes: &[u8]) -> Result<Self, HarnessError> {
        let mut cursor = Cursor::new(bytes);
        let length = cursor.read_short().map_err(IOError::ReadError)?;
        if length != 0x0E {
            return Err(HarnessError::PacketIOError(IOError::InvalidPacketLength(
                length,
            )));
        }
        let version = cursor.read_short().map_err(IOError::ReadError)?;
        let _sub_version = cursor.read_short().map_err(IOError::ReadError)?;
        let _server_type = cursor.read_byte().map_err(IOError::ReadError)?;
        let recv_iv = cursor.read_bytes(4).map_err(IOError::ReadError)?;
        let send_iv = cursor.read_bytes(4).map_err(IOError::ReadError)?;
        let locale = cursor.read_byte().map_err(IOError::ReadError)?;
        Ok(Self {
            version,
            recv_iv,
            send_iv,
            locale,
        })
    }
}
