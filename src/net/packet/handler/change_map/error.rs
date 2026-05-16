/* change_map/error.rs
 * The purpose of this module is to provide errors related to changing maps.
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

use crate::models::character::error::CharacterError;
use crate::models::map::error::MapError;
use crate::net::packet::codec::despawn_player::error::CodecDespawnPlayerError;
use crate::net::packet::codec::spawn_player::error::CodecSpawnPlayerError;
use crate::net::packet::io::error::IOError;
use crate::runtime::error::StateError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChangeMapError {
    #[error("Packet io error in change map layer")]
    IOError(#[from] IOError),

    #[error("Session error in change map layer")]
    SessionError(#[from] SessionError),

    #[error("Map model error in change map layer")]
    MapError(#[from] MapError),

    #[error("Character model error in change map layer")]
    CharacterError(#[from] CharacterError),

    #[error("State error in change map layer")]
    StateError(#[from] StateError),

    #[error("Codec spawn player packet error in change map layer")]
    CodecSpawnPlayerError(#[from] CodecSpawnPlayerError),

    #[error("Codec despawn player packet error in change map layer")]
    CodecDespawnPlayerError(#[from] CodecDespawnPlayerError),
}
