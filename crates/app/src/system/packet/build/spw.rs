/* spw/builder.rs
 * The purpose of this module is to build an outgoing login SPW packet.
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

use crate::system::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_spw_packet(status: bool) -> Result<&mut Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::CheckSpwResult as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet
        .write_byte(status as i16) // 0 failure 1 success
        .map_err(WriteError)?;
    Ok(packet)
}
