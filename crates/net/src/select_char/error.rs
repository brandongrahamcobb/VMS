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

use config::error::ConfigError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use state::error::StateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SelectCharError {
    #[error("Packet io error in select character without pic layer")]
    IOError(#[from] IOError),

    #[error("Session error in select character without pic layer")]
    SessionError(#[from] SessionError),

    #[error("Configuration error in select character without pic layer")]
    ConfigError(#[from] ConfigError),

    #[error("State error in select character without pic layer")]
    StateError(#[from] StateError),

    #[error("Packet build error in select character without pic layer")]
    PacketBuildError(#[from] PacketBuildError),
}
