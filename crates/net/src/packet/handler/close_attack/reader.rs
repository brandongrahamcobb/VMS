/* close_attack/reader.rs
 * The purpose of this module is to read an incoming close attack packet.
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

use crate::packet::handler::close_attack::error::CloseAttackError;
use packet::io::error::IOError::ReadError;
use packet::model::Packet;
use packet::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;

pub struct CloseAttackReader {
    pub count: i16,
    pub skill_id: i32,
    pub display: i16,
    pub toleft: i16,
    pub stance: i16,
    pub speed: i16,
    pub mob_damages: HashMap<u32, Vec<i32>>,
}

impl CloseAttackReader {
    pub fn read_close_attack_packet(packet: &Packet) -> Result<Self, CloseAttackError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let skip = 1;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let count = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let mobcount = count >> 4;
        let hitcount = count & 0x0F;
        let skill_id = pkt_reader.read_int().map_err(ReadError)?;
        let skip = 8;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let display = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let toleft = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let stance = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let skip = 1;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let speed = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let skip = 4;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let mut mob_damages = HashMap::new();
        for _ in 0..mobcount {
            let mob_id = pkt_reader.read_int().map_err(ReadError)? as u32;
            let skip = 14;
            pkt_reader.read_bytes(skip).map_err(ReadError)?;
            let mut damages: Vec<i32> = Vec::new();
            for _ in 0..hitcount {
                let dmg = pkt_reader.read_int().map_err(ReadError)?;
                damages.push(dmg);
            }
            if skill_id != 5221004 {
                let skip = 4;
                pkt_reader.read_bytes(skip).map_err(ReadError)?;
            }
            mob_damages.insert(mob_id, damages);
        }
        Ok(Self {
            count,
            skill_id,
            display,
            toleft,
            stance,
            speed,
            mob_damages,
        })
    }
}
