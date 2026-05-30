/* move_player/reader.rs
 * The purpose of this module is to read an incoming player movement packet.
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

use net::packet::model::Packet;

use crate::message::packet::player_moved::PlayerMovedMessage;
use crate::system::packet::dispatch::error::DispatchError;

const MOVEMENT_HEADER_LEN: usize = 9;

pub fn read_move_player_packet(
    packet: &Packet,
    client_id: i32,
) -> Result<PlayerMovedMessage, DispatchError> {
    let mut too_short: bool = false;
    if packet.bytes.len() <= 2 + MOVEMENT_HEADER_LEN {
        too_short = true;
    }
    let mut empty: bool = false;
    let movement_bytes = &packet.bytes[(2 + MOVEMENT_HEADER_LEN)..];
    if movement_bytes.is_empty() || movement_bytes[0] == 0 {
        empty = true;
    }
    Ok(PlayerMovedMessage {
        client_id,
        movement_bytes: movement_bytes.to_vec(),
        too_short,
        empty,
    })
}
