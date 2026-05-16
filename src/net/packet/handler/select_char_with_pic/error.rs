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

use crate::config::error::ConfigError;
use crate::models::account::error::AccountError;
use crate::net::packet::codec::login::error::CodecLoginError;
use crate::net::packet::io::error::IOError;
use crate::runtime::error::StateError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SelectCharWithPicError {
    #[error("Packet io error in select character with pic layer")]
    IOError(#[from] IOError),

    #[error("Session error in select character with pic layer")]
    SessionError(#[from] SessionError),

    #[error("Configuration error in select character with pic layer")]
    ConfigError(#[from] ConfigError),

    #[error("Account model error in select character with pic layer")]
    AccountError(#[from] AccountError),

    #[error("State error in select character with pic layer")]
    StateError(#[from] StateError),

    #[error("Codec login error in select character with pic layer")]
    CodecLoginError(#[from] CodecLoginError),
}
