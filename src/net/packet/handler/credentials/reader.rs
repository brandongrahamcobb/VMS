/* credentials/reader.rs
 * The purpose of this module is to read an incoming credentials validation packet.
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

use crate::inc::helpers;
use crate::net::packet::handler::credentials::error::CredentialsError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;


pub struct CredentialsReader {
    pub username: String,
    pub pw: String,
    pub hwid: String,
}

impl CredentialsReader {
    pub fn read_credentials_packet(packet: &Packet) -> Result<Self, CredentialsError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let username = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let pw = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let skip = 6;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let hwid_bytes = 4;
        let hwid = pkt_reader.read_bytes(hwid_bytes).map_err(ReadError)?;
        let hwid = helpers::to_hex_string(&hwid);
        Ok(Self { username, pw, hwid })
    }
}
