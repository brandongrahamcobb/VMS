/* credentials/error.rs
 * The purpose of this module is to provide errors related to credentials.
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
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CredentialsError {
    #[error("Packet io error in credentials layer")]
    IOError(#[from] IOError),

    #[error("Configuration error in credentials layer")]
    ConfigError(#[from] ConfigError),

    #[error("Account model error in credentials layer")]
    AccountError(#[from] AccountError),

    #[error("Session error in credentials layer")]
    SessionError(#[from] SessionError),

    #[error("Codec login error in credentials layer")]
    CodecLoginError(#[from] CodecLoginError),
}
