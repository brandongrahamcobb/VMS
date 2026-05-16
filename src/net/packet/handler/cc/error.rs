/* cc/error.rs
 * The purpose of this module is to provide errors related to changing channels.
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

use crate::config::error::ConfigError;
use crate::models::character::error::CharacterError;
use crate::models::map::error::MapError;
use crate::net::packet::codec::despawn_player::error::CodecDespawnPlayerError;
use crate::net::packet::codec::spawn_player::error::CodecSpawnPlayerError;
use crate::net::packet::io::error::IOError;
use crate::runtime::error::StateError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChangeChannelError {
    #[error("Packet io error in change channel layer")]
    IOError(#[from] IOError),

    #[error("Spawn player packet error in change channel layer")]
    CodecSpawnPlayerError(#[from] CodecSpawnPlayerError),

    #[error("Despawn player packet error in change channel layer")]
    CodecDespawnPlayerError(#[from] CodecDespawnPlayerError),

    #[error("Map model error in change channel layer")]
    MapError(#[from] MapError),

    #[error("Session error in change channel layer")]
    SessionError(#[from] SessionError),

    #[error("Configuration error in change channel layer")]
    ConfigError(#[from] ConfigError),

    #[error("State error in player logged in layer")]
    StateError(#[from] StateError),

    #[error("Character model error in change channel layer")]
    CharacterError(#[from] CharacterError),
}
