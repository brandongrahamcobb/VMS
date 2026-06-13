/* app/src/system/packet/build/error.rs
 * The purpose of this module is to provide errors related to packet building.
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

use core::num::TryFromIntError;
use std::time::SystemTimeError;

use config::error::ConfigError;
use net::packet::io::error::IOError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketBuildError {
    #[error("Packet io error in packet build layer")]
    IOError(#[from] IOError),

    #[error("Configuration error in packet build layer")]
    ConfigError(#[from] ConfigError),

    #[error("System time error in packet build layer")]
    SystemTimeError(#[from] SystemTimeError),

    #[error("Try from integer error in packet build layer")]
    TryFromIntError(#[from] TryFromIntError),
}
