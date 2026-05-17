/* execute/error.rs
 * The purpose of this module is to provide errors related to execution results.
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
use crate::net::packet::codec::spawn_mob::error::CodecSpawnMobError;
use crate::net::packet::codec::spawn_player::error::CodecSpawnPlayerError;
use crate::net::packet::model::Packet;
use crate::runtime::error::StateError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Error)]
pub enum ExecuteError {
    #[error("State error in execution layer")]
    StateError(#[from] StateError),

    #[error("Session error in execution layer")]
    SessionError(#[from] SessionError),

    #[error("Map model error in execution layer")]
    MapError(#[from] MapError),

    #[error("Failed UnboundedSender error in execution layer")]
    UnboundedSenderError(#[from] SendError<Packet>),

    #[error("Codec spawn player error in execution layer")]
    CodecSpawnPlayerError(#[from] CodecSpawnPlayerError),

    #[error("Codec spawn mob error in execution layer")]
    CodecMobPlayerError(#[from] CodecSpawnMobError),

    #[error("Character model error in execution layer")]
    CharacterError(#[from] CharacterError),

    #[error("Database error in execution layer")]
    DatabaseError(#[from] DatabaseError),
}
