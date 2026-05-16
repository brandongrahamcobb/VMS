/* delete_char/reader.rs
 * The purpose of this module is to read an incoming character deletion packet.
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

use crate::net::packet::handler::delete_char::error::DeleteCharError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct DeleteCharReader {
    pub char_id: i32,
    pub pic: String,
}

impl DeleteCharReader {
    pub fn read_delete_char_packet(packet: &Packet) -> Result<Self, DeleteCharError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let pic = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let char_id = pkt_reader.read_int().map_err(ReadError)?;
        Ok(Self { char_id, pic })
    }
}
