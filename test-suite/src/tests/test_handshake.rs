use config::settings;
use inc::helpers;
use packet;
use packet::constants::MAX_PACKET_LENGTH;
use packet::io::error::IOError;
use packet::io::read::PacketReader;
use packet::io::read::PktRead;
use packet::model::Packet;
use sec::aes::AES;
use sec::custom;
use std::collections::VecDeque;
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::error::HarnessError;

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
        let length = cursor.read_short().map_err(|e| IOError::ReadError(e))?;
        if length != 0x0E {
            return Err(HarnessError::PacketIOError(IOError::InvalidPacketLength(
                length,
            )));
        }
        let version = cursor.read_short().map_err(|e| IOError::ReadError(e))?;
        let _sub_version = cursor.read_short().map_err(|e| IOError::ReadError(e))?;
        let _server_type = cursor.read_byte().map_err(|e| IOError::ReadError(e))?;
        let recv_iv = cursor.read_bytes(4).map_err(|e| IOError::ReadError(e))?;
        let send_iv = cursor.read_bytes(4).map_err(|e| IOError::ReadError(e))?;
        let locale = cursor.read_byte().map_err(|e| IOError::ReadError(e))?;
        Ok(Self {
            version,
            recv_iv,
            send_iv,
            locale,
        })
    }
}

pub fn assert_handshake(version: i16, locale: u8) -> Result<(), HarnessError> {
    assert_eq!(version, 83);
    assert_eq!(locale, 8);
    Ok(())
}
