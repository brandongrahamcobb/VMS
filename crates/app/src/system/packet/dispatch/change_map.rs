/* change_map/reader.rs
 * The purpose of this module is to read an incoming map change packet.
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

use net::packet::io::error::IOError::ReadError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use std::io::Cursor;

use crate::message::packet::change_map::ReadChangeMapRequestMessage;
use crate::system::packet::dispatch::error::DispatchError;

pub fn read_change_map_packet(
    packet: &Packet,
    client_id: i32,
) -> Result<ReadChangeMapRequestMessage, DispatchError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let died: i16 = pkt_reader.read_byte().map_err(ReadError)? as i16;
    let tm: i32 = pkt_reader.read_int().map_err(ReadError)?;
    let pn: String = pkt_reader.read_str_with_length().map_err(ReadError)?;
    let skip: usize = 1;
    pkt_reader.read_bytes(skip).map_err(ReadError)?;
    let wheel_of_destiny_death_buff: i16 = pkt_reader.read_short().map_err(ReadError)?;
    Ok(ReadChangeMapRequestMessage {
        client_id,
        died,
        target_map: tm,
        portal_name: pn,
        wheel_of_destiny: wheel_of_destiny_death_buff,
    })
}
