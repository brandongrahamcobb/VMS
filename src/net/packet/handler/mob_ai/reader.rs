/* mob_ai/reader.rs
 * The purpose of this module is to read an incoming mob AI packets.
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

use crate::net::packet::handler::mob_ai::error::MobAiError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct MobAiReader {
    pub mob_id: u32,
    pub t: i16,
    pub skillb: u8,
    pub skill0: u8,
    pub skill1: u8,
    pub skill2: u8,
    pub skill3: u8,
    pub skill4: u8,
    pub origin_x: i16,
    pub origin_y: i16,
    pub command: u8,
    pub next_x: i16,
    pub next_y: i16,
    pub last_x: i16,
    pub last_y: i16,
    pub fh: u16,
    pub new_state: u8,
    pub duration: i16,
}

impl MobAiReader {
    pub fn read_mob_ai_packet(packet: &Packet) -> Result<Self, MobAiError> {
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
        let origin_x = pkt_reader.read_short().map_err(ReadError)?;
        let origin_y = pkt_reader.read_short().map_err(ReadError)?;
        let skip = 1;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let command = pkt_reader.read_byte().map_err(ReadError)?;
        let next_x = pkt_reader.read_short().map_err(ReadError)?;
        let next_y = pkt_reader.read_short().map_err(ReadError)?;
        let last_x = pkt_reader.read_short().map_err(ReadError)?;
        let last_y = pkt_reader.read_short().map_err(ReadError)?;
        let fh = pkt_reader.read_short().map_err(ReadError)? as u16;
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
            origin_x,
            origin_y,
            command,
            next_x,
            next_y,
            last_x,
            last_y,
            fh,
            new_state,
            duration,
        })
    }
}
