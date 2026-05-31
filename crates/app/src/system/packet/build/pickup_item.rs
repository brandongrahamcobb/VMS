/* pickup_item/builder.rs
 * The purpose of this module is to build an outgoing item pickup packet.
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

pub fn build_pickup_item_packet(
    char_id: i32,
    item_id: i32,
    pet_pickup: bool,
) -> Result<&mut Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::RemoveLoot as i16;
    packet.write_short(op).map_err(WriteError)?;
    let mode: u8 = 0;
    packet.write_byte(mode as i16).map_err(WriteError)?;
    packet.write_int(item_id).map_err(WriteError)?;
    if mode > 1 {
        packet.write_int(char_id).map_err(WriteError)?;
        if pet_pickup {
            packet.write_byte(pet_pickup as i16).map_err(WriteError)?;
        }
    }
    Ok(packet)
}
