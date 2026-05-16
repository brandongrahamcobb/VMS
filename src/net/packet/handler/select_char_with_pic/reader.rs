/* select_char_with_pic/reader.rs
 * The purpose of this module is to read an incoming, PIC, character selection packet.
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

use crate::net::packet::handler::select_char_with_pic::error::SelectCharWithPicError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct SelectCharWithPicReader {
    pub char_id: i32,
    pub mac: String,
    pub hwid: String,
    pub pic: String,
}

impl SelectCharWithPicReader {
    pub fn read_select_char_with_pic_packet(
        packet: &Packet,
    ) -> Result<Self, SelectCharWithPicError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let pic = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let char_id = pkt_reader.read_int().map_err(ReadError)?;
        let mac = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let hwid = pkt_reader.read_str_with_length().map_err(ReadError)?;
        Ok(Self {
            char_id,
            mac,
            hwid,
            pic,
        })
    }
}
