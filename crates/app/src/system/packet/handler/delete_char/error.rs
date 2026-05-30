/* delete_char/error.rs
 * The purpose of this module is to provide errors related to deleting characters.
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

use assembly::account::error::AccountAssemblyError;
use config::error::ConfigError;
use db::error::DatabaseError;
use entity::account::error::AccountEntityError;
use net::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError;
use session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeleteCharError {
    #[error("Packet io error in delete character layer")]
    IOError(#[from] IOError),

    #[error("Account entity error in delete character layer")]
    AccountEntityError(#[from] AccountEntityError),

    #[error("Session error in delete character layer")]
    SessionError(#[from] SessionError),

    #[error("Configuration error in delete character layer")]
    ConfigError(#[from] ConfigError),

    #[error("Database error in delete character layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Account assembly error in delete character layer")]
    AccountAssemblyError(#[from] AccountAssemblyError),

    #[error("Packet build error in delete character layer")]
    PacketBuildError(#[from] PacketBuildError),
}
