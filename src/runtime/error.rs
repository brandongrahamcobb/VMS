/* runtime/error.rs
 * The purpose of this module is to provide errors related to the runtime loop.
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
use crate::db::error::DatabaseError;
use crate::models::error::ModelError;
use crate::net::error::NetworkError;
use crate::net::packet::model::Packet;
use crate::runtime::session::error::SessionError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Config error in runtime layer")]
    ConfigError(#[from] ConfigError),

    #[error("Network error in runtime layer")]
    NetworkError(#[from] NetworkError),

    #[error("Concurrency join error in runtime layer")]
    JoinError(#[from] JoinError),

    #[error("Unexpected end of output in runtime layer")]
    UnexpectedOf(#[from] std::io::Error),

    #[error("Failed to connect to server in runtime layer")]
    FailedServerConnection(#[from] RuntimeServerConnectionError),

    #[error("Failed to create relay in runtime layer")]
    FailedRelayCreation(#[from] RuntimeRelayCreationError),

    #[error("Unexpected error in runtime layer")]
    UnexpectedError,

    #[error("Environment loading error in runtime layer")]
    DotenvError(#[from] dotenvy::Error),

    #[error("Session error in runtime layer")]
    SessionError(#[from] SessionError),

    #[error("Unsupported opcode error in runtime layer: {0} {1}")]
    UnsupportedOpcodeError(i16, String),

    #[error("Model error in runtime layer")]
    ModelError(#[from] ModelError),

    #[error("Database error in runtime layer")]
    DieselError(#[from] diesel::result::Error),

    #[error("Failed database in runtime layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Failed UnboundedSender error in runtime layer")]
    UnboundedSenderError(#[from] SendError<Packet>),
}

#[derive(Debug, Error)]
pub enum RuntimeServerConnectionError {
    #[error("Failed login server connection: {0}")]
    FailedLoginServerConnection(String),

    #[error("Failed world server connection: {0}")]
    FailedWorldServerConnection(String),
}

#[derive(Debug, Error)]
pub enum RuntimeRelayCreationError {
    #[error("Failed to create login relay: {0}")]
    FailedLoginRelayCreation(String),

    #[error("Failed to create world relay: {0}")]
    FailedWorldRelayCreation(String),
}
