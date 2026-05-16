/* list_chars/builder.rs
 * The purpose of this module is to build an outgoing character list packet.
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
use crate::net::packet::handler::list_chars::error::ListCharsError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_list_chars_packet(
        &mut self,
        chars: &Vec<Character>,
        channel_id: u8,
        char_slots: i16,
        pic_status: i16,
    ) -> Result<&mut Self, ListCharsError> {
        let op = SendOpcode::CharList as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(channel_id as i16).map_err(WriteError)?;
        self.write_byte(chars.len() as i16).map_err(WriteError)?;
        for char in chars.iter() {
            self.build_look_part_packet(&char)?;
        }
        self.write_byte(pic_status).map_err(WriteError)?;
        self.write_int(char_slots as i32).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_part_packet(&mut self, char: &Character) -> Result<&mut Self, ListCharsError> {
        self.build_list_char_meta_part_packet(char)?;
        self.build_look_meta_part_packet(char)?;
        self.write_byte(0).map_err(WriteError)?;
        // Disable rank.
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }
}
