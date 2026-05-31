/* packet/src/io/read.rs
 * The purpose of this module is to provide implementations for reading an incoming packet.
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

use crate::crypto::aes::AES;
use crate::crypto::custom;
use crate::packet::io::constants::HEADER_SIZE;
use crate::packet::io::error::IOError;
use crate::packet::io::error::IOError::ReadError;
use crate::packet::io::service;
use crate::packet::model::Packet;
use byteorder::{LittleEndian, ReadBytesExt};
use config::settings;
use std::io::Read;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

pub struct PacketReader {
    pkt_reader: BufReader<OwnedReadHalf>,
    aes: AES,
}

impl PacketReader {
    pub fn new(read_half: OwnedReadHalf, recv_iv: &[u8]) -> Result<Self, IOError> {
        Ok(Self {
            pkt_reader: BufReader::new(read_half),
            aes: AES::new(&recv_iv, settings::get_version()?),
        })
    }

    async fn read_buffer(&mut self, buf: &mut [u8]) -> Result<(), IOError> {
        self.pkt_reader.read_exact(buf).await.map_err(ReadError)?;
        Ok(())
    }

    async fn read_header(&mut self) -> Result<[u8; HEADER_SIZE as usize], IOError> {
        let mut buf = [0u8; HEADER_SIZE as usize];
        self.read_buffer(&mut buf).await?;
        service::check_header(&self.aes, &buf)?;
        Ok(buf)
    }

    async fn read_payload(&mut self, header: &[u8]) -> Result<Packet, IOError> {
        let length = get_packet_length(header);
        service::check_packet_length(length)?;
        let mut buf = vec![0u8; length as usize];
        self.read_buffer(&mut buf).await?;
        self.aes.crypt(&mut buf);
        custom::decrypt(&mut buf);
        Ok(Packet::new(&buf))
    }

    pub async fn read_packet(&mut self) -> Result<Packet, IOError> {
        let header: [u8; HEADER_SIZE as usize] = self.read_header().await?;
        let packet = self.read_payload(&header).await?;
        Ok(packet)
    }
}

pub trait PktRead: ReadBytesExt {
    fn read_byte(&mut self) -> std::io::Result<u8> {
        self.read_u8()
    }

    fn read_bytes(&mut self, length: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; length];
        match self.read_exact(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        }
    }

    fn read_short(&mut self) -> std::io::Result<i16> {
        self.read_i16::<LittleEndian>()
    }

    fn read_int(&mut self) -> std::io::Result<i32> {
        self.read_i32::<LittleEndian>()
    }

    fn read_long(&mut self) -> std::io::Result<i64> {
        self.read_i64::<LittleEndian>()
    }

    fn read_str(&mut self, length: usize) -> std::io::Result<String> {
        let mut buf = vec![0u8; length];
        match self.read_exact(&mut buf) {
            Ok(_) => match String::from_utf8(buf) {
                Ok(string) => Ok(string),
                Err(e) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                )),
            },
            Err(e) => Err(e),
        }
    }

    fn read_str_with_length(&mut self) -> std::io::Result<String> {
        match self.read_short() {
            Ok(length) => self.read_str(length as usize),
            Err(e) => Err(e),
        }
    }
}

impl<R: Read> PktRead for R {}

pub fn get_packet_length(header: &[u8]) -> i16 {
    (header[0] as i16 + ((header[1] as i16) << 8)) ^ (header[2] as i16 + ((header[3] as i16) << 8))
}
