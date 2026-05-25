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

use assembly::character::error::CharacterAssemblyError;
use entity::character::error::CharacterEntityError;
use entity::map::error::MapEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use state::error::StateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChangeMapEntityError {
    #[error("Packet io error in change map layer")]
    IOError(#[from] IOError),

    #[error("Session error in change map layer")]
    SessionError(#[from] SessionError),

    #[error("Map entity error in change map layer")]
    MapEntityError(#[from] MapEntityError),

    #[error("Character entity error in change map layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("State error in change map layer")]
    StateError(#[from] StateError),

    #[error("Character entity error in change map layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Packet build error in change map layer")]
    PacketBuildError(#[from] PacketBuildError),
}
