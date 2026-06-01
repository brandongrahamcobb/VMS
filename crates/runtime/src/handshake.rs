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

use crate::error::RuntimeError;
use config::settings;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;

pub async fn build_handshake_packet(
    recv_iv: [u8; 4],
    send_iv: [u8; 4],
) -> Result<Packet, RuntimeError> {
    let mut packet: Packet = Packet::new_empty();
    let version = settings::get_version()?; // should not be in here
    packet.write_short(0x0E).map_err(WriteError)?;
    packet.write_short(version).map_err(WriteError)?;
    // Not sure what this part is meant to represent...
    // HeavenClient doesn't seem to care for these values but the
    // official clients do...
    packet.write_short(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_bytes(recv_iv.to_vec()).map_err(WriteError)?;
    packet.write_bytes(send_iv.to_vec()).map_err(WriteError)?;
    packet
        .write_byte(8) // Locale byte
        .map_err(WriteError)?;
    Ok(packet)
}
