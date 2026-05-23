/* delet_char/builder.rs
 * The purpose of this module is to build an outgoing character deletion packet.
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
use op::send::SendOpcode;

impl Packet {
    pub fn build_delete_char_packet(
        &mut self,
        char_id: i32,
        pic_status: bool,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::DeleteCharacter as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char_id).map_err(WriteError)?;
        self.write_byte(pic_status as i16).map_err(WriteError)?;
        Ok(self)
    }
}
