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

use crate::db::error::DatabaseError;
use crate::models::character::error::CharacterError;
use crate::models::map::error::MapError;
use crate::net::packet::codec::player::error::CodecPlayerError;
use crate::net::packet::handler::change_map::error::ChangeMapError;
use crate::net::packet::io::error::IOError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TakeDamageError {
    #[error("Packet io error in take damage layer")]
    IOError(#[from] IOError),

    #[error("Session error in take damage layer")]
    SessionError(#[from] SessionError),

    #[error("Database error in take damage layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Character model error in take damage layer")]
    CharacterError(#[from] CharacterError),

    #[error("Map model error in take damage layer")]
    MapError(#[from] MapError),

    #[error("Codec player packet error in take damage layer")]
    CodecPlayerError(#[from] CodecPlayerError),

    #[error("Change map error in take damage layer")]
    ChangeMapError(#[from] ChangeMapError), // might be placed better if it was a shared place
}
