/* player_logged_in/builder.rs
 * The purpose of this module is to build an outgoing player login packet.
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

pub fn build_player_logged_in_keymap_packet(
    binds: &MapleKeybindings,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::KeyMap as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    let keybindings: Vec<&Keybinding> = (0..90).filter_map(|key| binds.0.get(&key)).collect();
    for bind in keybindings {
        packet.write_byte(bind.bind_type).map_err(WriteError)?;
        packet.write_int(bind.action).map_err(WriteError)?;
    }
    Ok(packet)
}
