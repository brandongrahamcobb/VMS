/* relay/types/error.rs
 * The purpose of this module is to provide errors related to relay implementations.
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

use crate::relay::execute::error::ExecuteError;
use net::packet::handler::error::PacketHandlerError;
use session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelayTypeError {
    #[error("Execute error in relay type layer")]
    ExecuteError(#[from] ExecuteError),

    #[error("Session error in relay type layer")]
    SessionError(#[from] SessionError),

    #[error("Unsupported opcode error in relay type layer: {0} {1}")]
    UnsupportedOpcodeError(i16, String),

    #[error("Packet handler error in relay type layer")]
    PacketHandlerError(#[from] PacketHandlerError),
}
