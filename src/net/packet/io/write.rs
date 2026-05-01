use crate::config::settings;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::sec::aes::AES;
use crate::sec::custom;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::tcp::OwnedWriteHalf;
use tracing::debug;

pub struct PacketWriter {
    pkt_writer: BufWriter<OwnedWriteHalf>,
    aes: AES,
}

impl PacketWriter {
    pub async fn new(write_half: OwnedWriteHalf, send_iv: &[u8]) -> Result<Self, NetworkError> {
        Ok(Self {
            pkt_writer: BufWriter::new(write_half),
            aes: AES::new(&send_iv.to_vec(), settings::get_version()?),
        })
    }

    pub async fn send_unencrypted_packet(
        &mut self,
        packet: &mut Packet,
    ) -> Result<(), NetworkError> {
        self.pkt_writer
            .write_all(&packet.bytes)
            .await
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.pkt_writer
            .flush()
            .await
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(())
    }

    pub async fn send_encrypted_packet(&mut self, packet: &mut Packet) -> Result<(), NetworkError> {
        let opcode = packet.opcode();
        let en = SendOpcode::from_i16(opcode).unwrap();
        debug!("Sent opcode: {} (0x{:02X}) ({:?})", opcode, opcode, en);
        let header = self.aes.gen_packet_header(packet.len() + 2);
        custom::encrypt(&mut packet.bytes);
        self.aes.crypt(&mut packet.bytes);
        self.pkt_writer
            .write_all(&header)
            .await
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.pkt_writer
            .write_all(&packet.bytes)
            .await
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.pkt_writer
            .flush()
            .await
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(())
    }
}

pub trait PktWrite: WriteBytesExt {
    fn write_byte(&mut self, byte: u8) -> std::io::Result<usize> {
        self.write(&[byte])
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.write(bytes)
    }

    fn write_short(&mut self, short: i16) -> std::io::Result<()> {
        self.write_u16::<LittleEndian>(short as u16)
    }

    fn write_int(&mut self, int: i32) -> std::io::Result<()> {
        self.write_u32::<LittleEndian>(int as u32)
    }

    fn write_long(&mut self, long: i64) -> std::io::Result<()> {
        self.write_u64::<LittleEndian>(long as u64)
    }

    fn write_str(&mut self, string: &str) -> std::io::Result<usize> {
        self.write(string.as_bytes())
    }

    fn write_str_with_length(&mut self, string: &str) -> std::io::Result<usize> {
        match self.write_short(string.len() as i16) {
            Ok(_) => self.write_str(string),
            Err(e) => Err(e),
        }
    }
}

impl<W: Write> PktWrite for W {}
