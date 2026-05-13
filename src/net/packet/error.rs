/* handler/error.rs
 * The purpose of this module is to provide errors related to handlers.
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

use crate::models::error::ModelError;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::io::error::IOError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Packet handler error in packet layer")]
    HandlerError(#[from] HandlerError),

    #[error("Packet io error in packet layer")]
    IOError(#[from] IOError),

    #[error("Invalid packet header in packet layer")]
    InvalidHeader,

    #[error("Invalid packet length in packet layer: {0}")]
    InvalidPacketLength(i16),

    #[error("Empty packet in packet layer")]
    EmptyPacket,

    #[error("Model error in packet layer")]
    ModelError(#[from] ModelError),
}
