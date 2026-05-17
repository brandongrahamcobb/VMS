/* spawn_mob/builder.rs
 * The purpose of this module is to build an outgoing spawn mob packet.
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

use crate::models::mob::wrapper::Mob;
use crate::net::packet::codec::spawn_mob::error::CodecSpawnMobError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_spawn_mob_packet(&mut self, mob: &Mob) -> Result<&mut Self, CodecSpawnMobError> {
        let op = SendOpcode::SpawnMob as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(mob.model.id as i32).map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 1];
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        self.write_int(mob.model.wz).map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 22];
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        self.write_short(mob.model.pos_x).map_err(WriteError)?;
        self.write_short(mob.model.pos_y).map_err(WriteError)?;
        let stance = 0 as i16; // 0 not sure
        self.write_byte(stance).map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 2];
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        self.write_short(mob.model.fh).map_err(WriteError)?;
        let effect: i16 = 0; // 0 = none
        self.write_byte(effect).map_err(WriteError)?;
        let team: i16 = -1; // -1 = no team
        self.write_byte(team).map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 4];
        self.write_bytes(skip).map_err(WriteError)?;
        Ok(self)
    }
}
