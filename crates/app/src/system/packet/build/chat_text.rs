/* chat_text/builder.rs
 * The purpose of this module is to build an outgoing general chat packet.
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

pub fn build_chat_text_packet(
    admin: bool,
    char_id: i32,
    msg: String,
    show: i16,
) -> Result<&mut Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ChatText as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(char_id).map_err(WriteError)?;
    packet.write_byte(admin as i16).map_err(WriteError)?;
    packet
        .write_str_with_length(msg.clone())
        .map_err(WriteError)?;
    packet.write_byte(show).map_err(WriteError)?;
    Ok(packet)
}
