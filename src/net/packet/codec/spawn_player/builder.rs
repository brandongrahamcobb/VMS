/* spawn_player/builder.rs
 * The purpose of this module is to build an outgoing spawn player packet.
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

use crate::models::character::wrapper::Character;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_spawn_player_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SpawnPlayer as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        let level = char.model.level as i16;
        self.write_byte(level).map_err(WriteError)?;
        self.write_str_with_length(char.model.ign.clone())
            .map_err(WriteError)?;
        let guild_name = String::from("Guild Name");
        self.write_str_with_length(guild_name).map_err(WriteError)?;
        let skip = 0 as i16;
        self.write_short(skip).map_err(WriteError)?; // guildlogobg
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; //guildlogobgcolor
        let skip = 0 as i16;
        self.write_short(skip).map_err(WriteError)?; //guildlogo
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; //guildlogocolor
        let skip = vec![0u8; 8];
        self.write_bytes(skip).map_err(WriteError)?;
        let morphed = 0; // 2 if morphed
        self.write_int(morphed).map_err(WriteError)?;
        let buff_mask_one = 0;
        self.write_int(buff_mask_one).map_err(WriteError)?;
        if buff_mask_one != 0 {
            if morphed == 2 {
                let buff_value = 0; // changes if morphed
                self.write_short(buff_value).map_err(WriteError)?;
            } else {
                let buff_value = 0; // changes if not morphed
                self.write_byte(buff_value).map_err(WriteError)?;
            }
        }
        let buff_mask_two = 0; // 0 not sure
        self.write_int(buff_mask_two).map_err(WriteError)?;
        let skip = vec![0u8; 43];
        self.write_bytes(skip).map_err(WriteError)?;
        let mount = 0; // 0 not sure
        self.write_int(mount).map_err(WriteError)?;
        let skip = vec![0u8; 61];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_short(char.model.job_wz).map_err(WriteError)?;
        self.build_look_meta_part_packet(char.clone())?;
        let count = 5110000;
        self.write_int(count).map_err(WriteError)?;
        let item_effect = 0; // 0 not sure
        self.write_int(item_effect).map_err(WriteError)?;
        let chair = 0; // 0 not sure
        self.write_int(chair).map_err(WriteError)?;
        let position_x = 0; // 0 this is a point so it might be wrong
        let position_y = 0; // 0 this is a point so it might be wrong
        self.write_short(position_x).map_err(WriteError)?;
        self.write_short(position_y).map_err(WriteError)?;
        let stance = 0 as i16; // 0 not sure
        self.write_byte(stance).map_err(WriteError)?;
        let skip = vec![0u8; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        for _ in 0..3 {
            let available = 0; // 0 not sure
            self.write_byte(available).map_err(WriteError)?;
            if available == 1 {
                let byte_two = 0 as i16; // 0 not sure
                self.write_byte(byte_two).map_err(WriteError)?;
                let pet_id = 0; // 0 is definitely not right
                self.write_int(pet_id).map_err(WriteError)?;
                let pet_name = String::from("George");
                self.write_str_with_length(pet_name).map_err(WriteError)?;
                let unique_id = 0; // 0 not sure
                self.write_int(unique_id).map_err(WriteError)?;
                let skip = 0;
                self.write_int(skip).map_err(WriteError)?;
                self.write_short(position_x).map_err(WriteError)?;
                self.write_short(position_y).map_err(WriteError)?;
                self.write_byte(stance).map_err(WriteError)?;
                let fhid = 0; // 0 not sure
                self.write_int(fhid).map_err(WriteError)?;
            } else {
                break;
            }
        }
        let mount_level = 0; // 0 not sure
        self.write_int(mount_level).map_err(WriteError)?;
        let mount_exp = 0; // 0 not sure
        self.write_int(mount_exp).map_err(WriteError)?;
        let mount_tiredness = 0; // 0 not sure
        self.write_int(mount_tiredness).map_err(WriteError)?;
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; // shop stuff
        let chalkboard_bool: bool = false; // false not sure
        let chalkboard: i16 = chalkboard_bool as i16; // false not sure
        self.write_byte(chalkboard).map_err(WriteError)?;
        if chalkboard_bool {
            let chalkboard_text = String::from("Placeholder");
            self.write_str_with_length(chalkboard_text)
                .map_err(WriteError)?;
        }
        let skip = vec![0u8; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        let team = 0 as i16; // 0 not sure
        self.write_byte(team).map_err(WriteError)?;
        Ok(self)
    }
}
