/* player_logged_in/builder.rs
 * The purpose of this module is to build an outgoing player login packet.
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
use crate::models::keybinding::wrapper::Keybinding;
use crate::net::packet::handler::player_logged_in::error::PlayerLoggedInError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use std::collections::HashMap;

impl Packet {
    pub fn build_player_logged_in_handler_keymap_packet(
        &mut self,
        binds: HashMap<i32, Keybinding>,
    ) -> Result<&mut Self, PlayerLoggedInError> {
        let op = SendOpcode::KeyMap as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        let keybindings: Vec<&Keybinding> = binds.values().collect();
        for bind in keybindings {
            self.write_byte(bind.model.bind_type as i16)
                .map_err(WriteError)?;
            self.write_int(bind.model.action as i32)
                .map_err(WriteError)?;
        }
        Ok(self)
    }

    pub fn build_set_field_packet(
        &mut self,
        char: Character,
        channel_id: u8,
    ) -> Result<&mut Self, PlayerLoggedInError> {
        let op = SendOpcode::SetField as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(channel_id as i32).map_err(WriteError)?;
        self //mode 1
            .write_byte(1)
            .map_err(WriteError)?;
        self //mode 2
            .write_byte(2)
            .map_err(WriteError)?;
        // Skip 23 bytes
        let skip = vec![0u8; 23];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        self.write_str(char.model.ign.clone()).map_err(WriteError)?;
        self.write_bytes(vec![0u8; 13 - char.model.ign.len()])
            .map_err(WriteError)?;
        let gender_wz = char.model.gender_wz as i16;
        self.write_byte(gender_wz).map_err(WriteError)?;
        let skin_wz = char.model.skin_wz as i16;
        self.write_byte(skin_wz).map_err(WriteError)?;
        self.write_int(char.model.face_wz).map_err(WriteError)?;
        self.write_int(char.model.hair_wz).map_err(WriteError)?;
        // Pets... Not implemented yet
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.build_player_logged_in_meta_part_packet(char.clone())?;
        Ok(self)
    }
}
