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

use assembly::character::error::CharacterAssemblyError;
use assembly::map::error::MapAssemblyError;
use db::error::DatabaseError;
use net::packet::build::error::PacketBuildError;
use net::packet::model::Packet;
use session::error::SessionError;
use state::error::StateError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Error)]
pub enum ExecuteError {
    #[error("State error in execution layer")]
    StateError(#[from] StateError),

    #[error("Session error in execution layer")]
    SessionError(#[from] SessionError),

    #[error("Map assembly error in execution layer")]
    MapAssemblyError(#[from] MapAssemblyError),

    #[error("Failed UnboundedSender error in execution layer")]
    UnboundedSenderError(#[from] SendError<Packet>),

    #[error("Character assembly error in execution layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Database error in execution layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Map not found in execution layer")]
    NoMap,

    #[error("Channel not found in execution layer")]
    NoChannel,

    #[error("World not found in execution layer")]
    NoWorld,

    #[error("Packet build error in execution layer")]
    PacketBuildError(#[from] PacketBuildError),
}
