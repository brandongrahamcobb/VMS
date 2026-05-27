/* create_char/builder.rs
 * The purpose of this module is to build an outgoing character creation packet.
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

use crate::build::error::PacketBuildError;
use crate::io::error::IOError::WriteError;
use crate::model::Packet;
use crate::prelude::*;
use entity::character::wrapper::Character;
use op::send::SendOpcode;

impl Packet {
    pub fn build_create_char_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::NewChar as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.build_new_character_look_part_packet(char)?;
        Ok(self)
    }

    fn build_new_character_look_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        self.build_list_char_meta_part_packet(char)?;
        self.build_new_character_look_meta_part_packet(char)?;
        self.write_byte(0).map_err(WriteError)?;
        // Disable rank.
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_new_character_look_meta_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        let gender_wz = char.model.gender_wz;
        self.write_byte(gender_wz).map_err(WriteError)?;
        let skin_wz = char.model.skin_wz as i16;
        self.write_byte(skin_wz).map_err(WriteError)?;
        self.write_int(char.model.face_wz).map_err(WriteError)?;
        self.write_byte(0) // megaphone
            .map_err(WriteError)?;
        self.write_int(char.model.hair_wz).map_err(WriteError)?;
        self.build_look_regular_equipment_part_packet(char)?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.build_look_cash_equipment_part_packet(char)?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.write_int(0) //maskedequips -111
            .map_err(WriteError)?;
        // Pet stuff...
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }
}
