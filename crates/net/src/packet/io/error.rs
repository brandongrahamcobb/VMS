/* io/error.rs
 * The purpose of this module is to provide errors related to reading/writing packets.
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
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IOError {
    #[error("Failed to read packet in packet io layer: {0}")]
    ReadError(std::io::Error),

    #[error("Failed to write to packet in packet io layer: {0}")]
    WriteError(std::io::Error),

    #[error("Invalid packet header in packet io layer")]
    InvalidHeader,

    #[error("Invalid packet length in packet io layer: {0}")]
    InvalidPacketLength(i16),

    #[error("Configuration error in packet io layer")]
    ConfigError(#[from] ConfigError),
}
