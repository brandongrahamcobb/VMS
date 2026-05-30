/* change_keymap/reader.rs
 * The purpose of this module is to read an incoming keymap change packet
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
use net::packet::model::Packet;
use net::packet::prelude::*;
use std::io::Cursor;

use crate::message::packet::change_keymap::ChangeKeymapMessage;
use crate::system::packet::dispatch::error::DispatchError;

pub fn read_change_keymap_packet(
    packet: &Packet,
    client_id: i32,
) -> Result<ChangeKeymapMessage, DispatchError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let _mode = pkt_reader.read_int().map_err(ReadError)?;
    let num_binds = pkt_reader.read_int().map_err(ReadError)?;
    let mut keys: Vec<i32> = Vec::new();
    let mut types: Vec<i16> = Vec::new();
    let mut model: Vec<i32> = Vec::new();
    for _ in 0..num_binds {
        keys.push(pkt_reader.read_int().map_err(ReadError)?);
        types.push(pkt_reader.read_byte().map_err(ReadError)? as i16);
        model.push(pkt_reader.read_int().map_err(ReadError)?);
    }
    Ok(ChangeKeymapMessage {
        client_id,
        keys,
        types,
        model,
    })
}
