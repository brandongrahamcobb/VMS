/* register_pic/reader.rs
 * The purpose of this module is to read an incoming PIC registration packet.
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

use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct RegisterPicReader {
    pub char_id: i32,
    pub mac: String,
    pub hwid: String,
    pub pic: String,
}

impl RegisterPicReader {
    pub fn read_register_pic_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let skip = 1;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let char_id = pkt_reader.read_int().map_err(ReadError)?;
        let mac = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let hwid = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let pic = pkt_reader.read_str_with_length().map_err(ReadError)?;
        Ok(Self {
            char_id,
            mac,
            hwid,
            pic,
        })
    }
}
