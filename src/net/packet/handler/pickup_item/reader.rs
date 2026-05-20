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

use crate::models::map::model::Point;
use crate::net::packet::handler::pickup_item::error::PickupItemError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct PickupItemReader {
    pub item_id: i32,
    pub pos: Point,
}

impl PickupItemReader {
    pub fn read_pickup_item_packet(packet: &Packet) -> Result<Self, PickupItemError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        pkt_reader.read_int().map_err(ReadError)?;
        pkt_reader.read_byte().map_err(ReadError)?;
        let x = pkt_reader.read_short().map_err(ReadError)?;
        let y = pkt_reader.read_short().map_err(ReadError)?;
        let item_id: i32 = pkt_reader.read_int().map_err(ReadError)?;
        Ok(Self {
            pos: Point { x, y },
            item_id,
        })
    }
}
