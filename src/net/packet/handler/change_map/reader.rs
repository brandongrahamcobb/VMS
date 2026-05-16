/* change_map/reader.rs
 * The purpose of this module is to read an incoming map change packet.
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

use crate::net::packet::handler::change_map::error::ChangeMapError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;


pub struct ChangeMapReader {
    pub died: i16,
    pub tm: i32,
    pub tn: String,
    pub wod: i16,
}

impl ChangeMapReader {
    pub fn read_change_map_packet(packet: &Packet) -> Result<Self, ChangeMapError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let died: i16 = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let tm: i32 = pkt_reader.read_int().map_err(ReadError)?;
        let tn: String = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let skip: usize = 1;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let wheel_of_destiny_death_buff: i16 = pkt_reader.read_short().map_err(ReadError)? as i16;
        Ok(Self {
            died,
            tm,
            tn,
            wod: wheel_of_destiny_death_buff,
        })
    }
}
