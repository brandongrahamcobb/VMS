/* close_attack/builder.rs
 * The purpose of this module is to build an outgoing close attack packet.
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

use crate::net::packet::handler::take_damage::error::TakeDamageError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_take_damage_packet(&mut self, hp: i16) -> Result<&mut Self, TakeDamageError> {
        let op = SendOpcode::ChangeStats as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0i16).map_err(WriteError)?; // itemreaction
        self.write_int(0x400i32).map_err(WriteError)?; // updatemask: HP
        self.write_short(hp).map_err(WriteError)?;
        Ok(self)
    }
}
// 0x1       SKIN
// 0x2       FACE
// 0x4       HAIR
// 0x10      LEVEL
// 0x20      JOB
// 0x40      STR
// 0x80      DEX
// 0x100     INT
// 0x200     LUK
// 0x400     HP
// 0x800     MAXHP
// 0x1000    MP
// 0x2000    MAXMP
// 0x4000    AP
// 0x8000    SP
// 0x10000   EXP
// 0x20000   FAME
// 0x40000   MESO
// 0x180008  PET
// 0x200000  GACHAEXP
