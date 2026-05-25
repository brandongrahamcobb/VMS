/* take_damage/error.rs
 * The purpose of this module is to provide errors related to taking damage.
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

use crate::change_map::error::ChangeMapEntityError;
use assembly::character::error::CharacterAssemblyError;
use db::error::DatabaseError;
use entity::map::error::MapEntityError;
use metadata::map::error::MapMetadataError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TakeDamageError {
    #[error("Packet io error in take damage layer")]
    IOError(#[from] IOError),

    #[error("Session error in take damage layer")]
    SessionError(#[from] SessionError),

    #[error("Database error in take damage layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Character entity error in take damage layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Map entity error in take damage layer")]
    MapEntityError(#[from] MapEntityError),

    #[error("Change map error in take damage layer")]
    ChangeMapEntityError(#[from] ChangeMapEntityError), // might be placed better if it was a shared place

    #[error("Map metadata error in take damage layer")]
    MapMetadataError(#[from] MapMetadataError),

    #[error("Packet build error in take damage layer")]
    PacketBuildError(#[from] PacketBuildError),
}
