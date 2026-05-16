/* packet/service.rs
 * The purpose of this module is to provide assisting functions and implementations for packets.
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

use crate::constants::MAX_PACKET_LENGTH;
use crate::net::packet::io::error::IOError;
use crate::sec::aes::AES;

pub fn check_header(aes: &AES, header: &[u8]) -> Result<(), IOError> {
    if !(((header[0] ^ aes.iv[2]) & 0xFF) == ((aes.version >> 8) as u8 & 0xFF)
        && ((header[1] ^ aes.iv[3]) & 0xFF) == (aes.version & 0xFF) as u8)
    {
        return Err(IOError::InvalidHeader);
    }
    Ok(())
}

pub fn check_packet_length(length: i16) -> Result<(), IOError> {
    if length < 2 || length > MAX_PACKET_LENGTH {
        return Err(IOError::InvalidPacketLength(length));
    }
    Ok(())
}
