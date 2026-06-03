/* register_pic/reader.rs
 * The purpose of this module is to read an incoming PIC registration packet.
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

use crate::message::packet::register_pic::ReadRegisterPicMessage;
use crate::system::packet::dispatch::error::DispatchError;
use net::packet::io::error::IOError::ReadError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use std::io::Cursor;

pub fn read_register_pic_packet(
    packet: &Packet,
    client_id: i32,
) -> Result<ReadRegisterPicMessage, DispatchError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let skip = 1;
    pkt_reader.read_bytes(skip).map_err(ReadError)?;
    let char_id = pkt_reader.read_int().map_err(ReadError)?;
    let mac = pkt_reader.read_str_with_length().map_err(ReadError)?;
    let hwid = pkt_reader.read_str_with_length().map_err(ReadError)?;
    let pic = pkt_reader.read_str_with_length().map_err(ReadError)?;
    Ok(ReadRegisterPicMessage {
        client_id,
        char_id,
        mac,
        hwid,
        pic,
    })
}
