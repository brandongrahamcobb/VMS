/* move_player/reader.rs
 * The purpose of this module is to read an incoming player movement packet.
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

use crate::net::packet::handler::move_mob::error::MoveMobError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct MoveMobReader {
    pub mob_id: u32,
    pub t: i16,
    pub skillb: u8,
    pub skill0: u8,
    pub skill1: u8,
    pub skill2: u8,
    pub skill3: u8,
    pub skill4: u8,
    pub x: i16,
    pub y: i16,
    pub command: u8,
    pub pos_x: i16,
    pub pos_y: i16,
    pub last_x: i16,
    pub last_y: i16,
    pub fh: i16,
    pub new_state: u8,
    pub duration: i16,
}

impl MoveMobReader {
    pub fn read_move_mob_packet(packet: &Packet) -> Result<Self, MoveMobError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let mob_id = pkt_reader.read_int().map_err(ReadError)?;
        let t = pkt_reader.read_short().map_err(ReadError)?;
        let skillb = pkt_reader.read_byte().map_err(ReadError)?;
        let skill0 = pkt_reader.read_byte().map_err(ReadError)?;
        let skill1 = pkt_reader.read_byte().map_err(ReadError)?;
        let skill2 = pkt_reader.read_byte().map_err(ReadError)?;
        let skill3 = pkt_reader.read_byte().map_err(ReadError)?;
        let skill4 = pkt_reader.read_byte().map_err(ReadError)?;
        let skip = 13;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let x = pkt_reader.read_short().map_err(ReadError)?;
        let y = pkt_reader.read_short().map_err(ReadError)?;
        let skip = 1;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let command = pkt_reader.read_byte().map_err(ReadError)?;
        let pos_x = pkt_reader.read_short().map_err(ReadError)?;
        let pos_y = pkt_reader.read_short().map_err(ReadError)?;
        let last_x = pkt_reader.read_short().map_err(ReadError)?;
        let last_y = pkt_reader.read_short().map_err(ReadError)?;
        let fh = pkt_reader.read_short().map_err(ReadError)?;
        let new_state = pkt_reader.read_byte().map_err(ReadError)?;
        let duration = pkt_reader.read_short().map_err(ReadError)?;
        Ok(Self {
            mob_id: mob_id as u32,
            t,
            skillb,
            skill0,
            skill1,
            skill2,
            skill3,
            skill4,
            x,
            y,
            command,
            pos_x,
            pos_y,
            last_x,
            last_y,
            fh,
            new_state,
            duration,
        })
    }
}
