/* handshake/builder.rs
 * The purpose of this module is to build an outgoing handshake packet.
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

use crate::config::settings;
use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::prelude::*;

impl Packet {
    pub async fn build_handshake_packet(
        &mut self,
        recv_iv: [u8; 4],
        send_iv: [u8; 4],
    ) -> Result<&mut Self, NetworkError> {
        let version = settings::get_version()?; // should not be in here
        self.write_short(0x0E).map_err(WriteError)?;
        self.write_short(version).map_err(WriteError)?;
        // Not sure what this part is meant to represent...
        // HeavenClient doesn't seem to care for these values but the
        // official clients do...
        self.write_short(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_bytes(recv_iv.to_vec()).map_err(WriteError)?;
        self.write_bytes(send_iv.to_vec()).map_err(WriteError)?;
        self.write_byte(8) // Locale byte
            .map_err(WriteError)?;
        Ok(self)
    }
}
