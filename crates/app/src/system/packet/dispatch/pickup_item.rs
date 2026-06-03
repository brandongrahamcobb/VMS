/* pickup_item/reader.rs
 * The purpose of this module is to read an incoming item pickup packet.
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

use base::map::Point;
use net::packet::io::error::IOError::ReadError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use std::io::Cursor;

use crate::message::packet::pickup_item::ReadPickupItemRequestMessage;
use crate::system::packet::dispatch::error::DispatchError;

pub fn read_pickup_item_packet(
    packet: &Packet,
    client_id: i32,
) -> Result<ReadPickupItemRequestMessage, DispatchError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    pkt_reader.read_int().map_err(ReadError)?;
    pkt_reader.read_byte().map_err(ReadError)?;
    let x = pkt_reader.read_short().map_err(ReadError)?;
    let y = pkt_reader.read_short().map_err(ReadError)?;
    let item_id: i32 = pkt_reader.read_int().map_err(ReadError)?;
    Ok(ReadPickupItemRequestMessage {
        client_id,
        pos: Point { x, y },
        item_id,
        pet_pickup: false, // placeholder
    })
}
