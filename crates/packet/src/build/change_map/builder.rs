/* change_map/builder.rs
 * The purpose of this module is to build an outgoing map change packet.
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
    pub fn build_set_field_change_map_packet(
        &mut self,
        channel_id: u8,
        map_wz: i32,
        pid: u8,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::SetField as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(channel_id as i32).map_err(WriteError)?;
        self //mode 1
            .write_byte(0)
            .map_err(WriteError)?;
        self //mode 2
            .write_byte(0)
            .map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_int(map_wz).map_err(WriteError)?;
        self.write_byte(pid as i16).map_err(WriteError)?;
        Ok(self)
    }
}
