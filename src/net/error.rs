/* packet/error.rs
 * The purpose of this module is to provide errors related to packets.
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
use crate::models::account::error::AccountError;
use crate::models::character::error::CharacterError;
use crate::models::error::ModelError;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError;
use crate::net::packet::model::Packet;
use crate::runtime::session::error::SessionError;
use bcrypt::BcryptError;
use core::num::ParseIntError;
use std::time::SystemTimeError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Config error in network layer")]
    ConfigError(#[from] ConfigError),

    #[error("Packet error in network layer")]
    PacketError(#[from] PacketError),

    #[error("Database error in network layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("System time error in network layer")]
    SystemTimeError(#[from] SystemTimeError),

    #[error("Unexpected error in network layer")]
    UnexpectedError,

    #[error("Integer conversion error in network layer")]
    IntConversion(#[from] std::num::TryFromIntError),

    #[error("Bcrypt error in network layer")]
    CryptError(#[from] BcryptError),

    #[error("Session error in network layer")]
    SessionError(#[from] SessionError),

    #[error("Model error in network layer")]
    ModelError(#[from] ModelError),

    #[error("Entry in database not found in network layer")]
    DieselError(#[from] diesel::result::Error),

    #[error("IO error in network layer")]
    IOError(#[from] IOError),

    #[error("Account error in network layer")]
    AccountError(#[from] AccountError),

    #[error("Character error in network layer")]
    CharacterError(#[from] CharacterError),

    #[error("Failed UnboundedSender error in runtime layer")]
    UnboundedSenderError(#[from] SendError<Packet>),

    #[error("Parse int error in runtime layer")]
    ParseIntError(#[from] ParseIntError),
}
