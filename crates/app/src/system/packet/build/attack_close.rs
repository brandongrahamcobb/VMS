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

use crate::system::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError::WriteError;
use net::packet::model::Packet;
use net::packet::io::prelude::*;
use op::send::SendOpcode;
use std::collections::HashMap;

impl Packet {
    pub fn build_close_attack_packet(
        &mut self,
        char_id: i32,
        count: i16,
        skill_level: i16,
        skill_id: i32,
        display: i16,
        toleft: i16,
        stance: i16,
        speed: i16,
        mob_damages: HashMap<u32, Vec<i32>>,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::AttackedClose as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char_id).map_err(WriteError)?;
        self.write_byte(count).map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 1];
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        self.write_byte(skill_level).map_err(WriteError)?;
        self.write_int(skill_id).map_err(WriteError)?;
        self.write_byte(display).map_err(WriteError)?;
        self.write_byte(toleft).map_err(WriteError)?;
        self.write_byte(stance).map_err(WriteError)?;
        self.write_byte(speed).map_err(WriteError)?;
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        for (mob_id, damages) in mob_damages {
            self.write_int(mob_id as i32).map_err(WriteError)?;
            self.write_bytes(skip.clone()).map_err(WriteError)?;
            let meso_explosion_skill_id: i32 = 4211006;
            if skill_id == meso_explosion_skill_id {
                let placeholder_number_of_mesos: i16 = 15;
                let max_hits: i16 =
                    get_max_number_of_meso_explosion_hits(skill_level, placeholder_number_of_mesos);
                self.write_byte(max_hits).map_err(WriteError)?;
            }
            for dmg in damages {
                self.write_int(dmg).map_err(WriteError)?;
            }
        }
        Ok(self)
    }
}

pub fn get_max_number_of_meso_explosion_hits(skill_level: i16, number_of_meso_chunks: i16) -> i16 {
    let skill_max: i16 = match skill_level {
        1 => 5,
        2 | 3 => 6,
        4 | 5 => 7,
        6 | 7 => 8,
        8 | 9 => 9,
        10 | 11 => 10,
        12 | 13 => 11,
        14 | 15 => 12,
        16 | 17 => 13,
        18 | 19 => 14,
        20 => 15,
        _ => 0,
    };
    if number_of_meso_chunks < skill_max {
        number_of_meso_chunks
    } else {
        skill_max
    }
}
