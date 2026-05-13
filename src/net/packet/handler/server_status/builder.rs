/* server_status/builder.rs
 * The purpose of this module is to build an outgoing server status packet.
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

use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_server_status_packet(
        &mut self,
        status: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ServerStatus as i16;
        self.write_short(op).map_err(WriteError)?;
        let status = status as i16;
        self.write_short(status) // Highly populated status!
            .map_err(WriteError)?;
        Ok(self)
    }
}
