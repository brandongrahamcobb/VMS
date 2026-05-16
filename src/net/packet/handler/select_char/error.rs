/* select_char/error.rs
 * The purpose of this module is to provide errors related to selecting a character without a pic.
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
use crate::net::packet::codec::login::error::CodecLoginError;
use crate::net::packet::io::error::IOError;
use crate::runtime::error::StateError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SelectCharError {
    #[error("Packet io error in select character without pic layer")]
    IOError(#[from] IOError),

    #[error("Session error in select character without pic layer")]
    SessionError(#[from] SessionError),

    #[error("Configuration error in select character without pic layer")]
    ConfigError(#[from] ConfigError),

    #[error("Codec login error in select character without pic layer")]
    CodecLoginError(#[from] CodecLoginError),

    #[error("State error in select character without pic layer")]
    StateError(#[from] StateError),
}
