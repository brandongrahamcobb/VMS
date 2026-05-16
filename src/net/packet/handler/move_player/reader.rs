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

use crate::net::packet::handler::move_player::error::MovePlayerError;
use crate::net::packet::model::Packet;

const MOVEMENT_HEADER_LEN: usize = 9;

#[derive(Clone)]
pub struct MovePlayerReader {
    pub movement_bytes: Vec<u8>,
    pub too_short: bool,
    pub empty: bool,
}

impl MovePlayerReader {
    pub fn read_move_player_packet(packet: &Packet) -> Result<Self, MovePlayerError> {
        let mut too_short: bool = false;
        if packet.bytes.len() <= 2 + MOVEMENT_HEADER_LEN {
            too_short = true;
        }
        let mut empty: bool = false;
        let movement_bytes = &packet.bytes[(2 + MOVEMENT_HEADER_LEN)..];
        if movement_bytes.is_empty() || movement_bytes[0] == 0 {
            empty = true;
        }
        Ok(Self {
            movement_bytes: movement_bytes.to_vec(),
            too_short,
            empty,
        })
    }
}
