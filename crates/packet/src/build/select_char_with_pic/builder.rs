/* select_char_with_pic/builder.rs
 * The purpose of this module is to build an outgoing, PIC, character selection packet.
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
    pub fn build_select_char_handler_failed_pic_packet(
        &mut self,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::CheckSpwResult as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0) // 0 for failure, anything else for success
            .map_err(WriteError)?;
        Ok(self)
    }
}
