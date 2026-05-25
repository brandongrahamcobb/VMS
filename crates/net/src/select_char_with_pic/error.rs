/* select_char_with_pic/error.rs
 * The purpose of this module is to provide errors related to selecting a character with a pic.
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
use entity::account::error::AccountEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use state::error::StateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SelectCharWithPicError {
    #[error("Packet io error in select character with pic layer")]
    IOError(#[from] IOError),

    #[error("Session error in select character with pic layer")]
    SessionError(#[from] SessionError),

    #[error("Configuration error in select character with pic layer")]
    ConfigError(#[from] ConfigError),

    #[error("Account entity error in select character with pic layer")]
    AccountEntityError(#[from] AccountEntityError),

    #[error("State error in select character with pic layer")]
    StateError(#[from] StateError),

    #[error("Account assembly error in select character with pic layer")]
    AccountAssemblyError(#[from] AccountAssemblyError),

    #[error("Packet build error in select character with pic layer")]
    PacketBuildError(#[from] PacketBuildError),
}
