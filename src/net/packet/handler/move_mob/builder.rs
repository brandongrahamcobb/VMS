/* move_mob/builder.rs
 * The purpose of this module is to build an outgoing mob movement packet.
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
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_mob_move_packet(
        &mut self,
        mob_id: u32,
        skillb: u8,
        skill0: u8,
        skill1: u8,
        skill2: u8,
        skill3: u8,
        skill4: u8,
        pos_x: i16,
        pos_y: i16,
        command: u8,
        x: i16,
        y: i16,
        last_x: i16,
        last_y: i16,
        fh: i16,
        new_state: u8,
        duration: i16,
    ) -> Result<&mut Self, MoveMobError> {
        let op = SendOpcode::MoveMonster as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(mob_id as i32).map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 1];
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        self.write_byte(skillb as i16).map_err(WriteError)?;
        self.write_byte(skill0 as i16).map_err(WriteError)?;
        self.write_byte(skill1 as i16).map_err(WriteError)?;
        self.write_byte(skill2 as i16).map_err(WriteError)?;
        self.write_byte(skill3 as i16).map_err(WriteError)?;
        self.write_byte(skill4 as i16).map_err(WriteError)?;
        self.write_short(pos_x).map_err(WriteError)?;
        self.write_short(pos_y).map_err(WriteError)?;
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        self.write_byte(command as i16).map_err(WriteError)?;
        self.write_short(x).map_err(WriteError)?;
        self.write_short(y).map_err(WriteError)?;
        self.write_short(last_x).map_err(WriteError)?;
        self.write_short(last_y).map_err(WriteError)?;
        self.write_short(fh).map_err(WriteError)?;
        self.write_byte(new_state as i16).map_err(WriteError)?;
        self.write_short(duration).map_err(WriteError)?;
        Ok(self)
    }
}
