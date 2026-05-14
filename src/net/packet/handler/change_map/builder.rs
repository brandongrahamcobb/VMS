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

use crate::models::channel::wrapper::Channel;
use crate::models::map::wrapper::Map;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_set_field_change_map_packet(
        &mut self,
        channel: Channel,
        map: Map,
        pid: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SetField as i16;
        self.write_short(op).map_err(WriteError)?;
        let channel_id = channel.model.id as i32;
        self.write_int(channel_id).map_err(WriteError)?;
        self //mode 1
            .write_byte(0)
            .map_err(WriteError)?;
        self //mode 2
            .write_byte(0)
            .map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_int(map.model.wz).map_err(WriteError)?;
        self.write_byte(pid).map_err(WriteError)?;
        Ok(self)
    }
}
