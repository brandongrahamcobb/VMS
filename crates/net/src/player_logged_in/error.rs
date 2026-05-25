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

use assembly::character::error::CharacterAssemblyError;
use assembly::keybinding::error::KeybindingAssemblyError;
use entity::character::error::CharacterEntityError;
use entity::keybinding::error::KeybindingEntityError;
use entity::map::error::MapEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use state::error::StateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerLoggedInError {
    #[error("Packet io error in player logged in layer")]
    IOError(#[from] IOError),

    #[error("Map entity error in player logged in layer")]
    MapEntityError(#[from] MapEntityError),

    #[error("Session error in player logged in layer")]
    SessionError(#[from] SessionError),

    #[error("Keybinding entity error in player logged in layer")]
    KeybindingEntityError(#[from] KeybindingEntityError),

    #[error("Keybinding assembly error in player logged in layer")]
    KeybindingAssemblyError(#[from] KeybindingAssemblyError),

    #[error("Character assembly error in player logged in layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Character entity error in player logged in layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("State error in player logged in layer")]
    StateError(#[from] StateError),

    #[error("Packet build error in player logged in layer")]
    PacketBuildError(#[from] PacketBuildError),
}
