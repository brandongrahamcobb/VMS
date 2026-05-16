/* player_logged_in/error.rs
 * The purpose of this module is to provide errors related to players logging in.
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
use crate::models::keybinding::error::KeybindingError;
use crate::models::map::error::MapError;
use crate::net::packet::codec::login::error::CodecLoginError;
use crate::net::packet::codec::spawn_player::error::CodecSpawnPlayerError;
use crate::net::packet::io::error::IOError;
use crate::runtime::error::StateError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerLoggedInError {
    #[error("Packet io error in player logged in layer")]
    IOError(#[from] IOError),

    #[error("Spawn player packet error in player logged in layer")]
    CodecSpawnPlayerError(#[from] CodecSpawnPlayerError),

    #[error("Map model error in player logged in layer")]
    MapError(#[from] MapError),

    #[error("Session error in player logged in layer")]
    SessionError(#[from] SessionError),

    #[error("Keybinding model error in player logged in layer")]
    KeybindingError(#[from] KeybindingError),

    #[error("Codec login error in player logged in layer")]
    CodecLoginError(#[from] CodecLoginError),

    #[error("Character model error in player logged in layer")]
    CharacterError(#[from] CharacterError),

    #[error("State error in player logged in layer")]
    StateError(#[from] StateError),
}
