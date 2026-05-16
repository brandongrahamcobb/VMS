/* write.rs
 * The purpose of this module is to provide implementations for writing an outgoing packet.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::config::settings;
use crate::net::packet::io::error::IOError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
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
    pub async fn new(write_half: OwnedWriteHalf, send_iv: &[u8]) -> Result<Self, IOError> {
        Ok(Self {
            pkt_writer: BufWriter::new(write_half),
            aes: AES::new(&send_iv.to_vec(), settings::get_version()?),
        })
    }

    pub async fn send_unencrypted_packet(&mut self, packet: &Packet) -> Result<(), IOError> {
        self.pkt_writer
            .write_all(&packet.bytes)
            .await
            .map_err(WriteError)?;
        self.pkt_writer.flush().await.map_err(WriteError)?;
        Ok(())
    }

    pub async fn send_encrypted_packet(&mut self, packet: &mut Packet) -> Result<(), IOError> {
        let opcode = packet.opcode();
        let en = SendOpcode::from_i16(opcode).unwrap();
        debug!("Sent opcode: {} (0x{:02X}) ({:?})", opcode, opcode, en);
        let header = self.aes.gen_packet_header(packet.len() + 2);
        custom::encrypt(&mut packet.bytes);
        self.aes.crypt(&mut packet.bytes);
        self.pkt_writer
            .write_all(&header)
            .await
            .map_err(WriteError)?;
        self.pkt_writer
            .write_all(&packet.bytes)
            .await
            .map_err(WriteError)?;
        self.pkt_writer.flush().await.map_err(WriteError)?;
        Ok(())
    }
}

pub trait PktWrite: WriteBytesExt {
    fn write_bytes(&mut self, bytes: Vec<u8>) -> std::io::Result<usize> {
        self.write(&bytes)
    }

    fn write_byte(&mut self, byte: i16) -> std::io::Result<()> {
        self.write_u8(byte as u8)
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

    fn write_str(&mut self, string: String) -> std::io::Result<usize> {
        self.write(string.as_bytes())
    }

    fn write_str_with_length(&mut self, string: String) -> std::io::Result<usize> {
        match self.write_short(string.len() as i16) {
            Ok(_) => self.write_str(string),
            Err(e) => Err(e),
        }
    }
}

impl<W: Write> PktWrite for W {}
