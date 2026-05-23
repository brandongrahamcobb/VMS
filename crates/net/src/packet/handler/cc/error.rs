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

use assembly::character::error::CharacterAssemblyError;
use config::error::ConfigError;
use entity::character::error::CharacterEntityError;
use entity::map::error::MapEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use state::error::StateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChangeChannelEntityError {
    #[error("Packet io error in change channel layer")]
    IOError(#[from] IOError),

    #[error("Map entity error in change channel layer")]
    MapEntityError(#[from] MapEntityError),

    #[error("Session error in change channel layer")]
    SessionError(#[from] SessionError),

    #[error("Configuration error in change channel layer")]
    ConfigError(#[from] ConfigError),

    #[error("State error in player logged in layer")]
    StateError(#[from] StateError),

    #[error("Character entity error in change channel layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("Character entity error in change channel layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Packet build error in change channel layer")]
    PacketBuildError(#[from] PacketBuildError),
}
