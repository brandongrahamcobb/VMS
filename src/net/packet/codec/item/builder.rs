/* item/builder.rs
 * The purpose of this module is to build an outgoing item packet.
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
use crate::net::packet::codec::item::error::CodecItemError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_drop_loot_packet(
        &mut self,
        mode: u8, // animation 0 fade, 1 drop mob, 2 spawn in
        id: u32,
        is_meso: bool,
        wz_or_meso_amount: i32,
        owner: i32,     // char id or 0
        can_pickup: u8, // 0 everyone 1 owner, 2 party
        drop_to: Point,
        drop_from: Point,
        player_drop: bool,
    ) -> Result<&mut Self, CodecItemError> {
        let op = SendOpcode::DropLoot as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(mode as i16).map_err(WriteError)?;
        self.write_int(id as i32).map_err(WriteError)?;
        dbg!(id);
        dbg!("DROP");
        self.write_byte(is_meso as i16).map_err(WriteError)?;
        self.write_int(wz_or_meso_amount).map_err(WriteError)?;
        self.write_int(owner).map_err(WriteError)?;
        self.write_byte(can_pickup as i16).map_err(WriteError)?;
        self.write_short(drop_to.x).map_err(WriteError)?;
        self.write_short(drop_to.y).map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 4];
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        if mode != 2 {
            self.write_short(drop_from.x).map_err(WriteError)?;
            self.write_short(drop_from.y).map_err(WriteError)?;
            let skip: Vec<u8> = vec![0; 2];
            self.write_bytes(skip.clone()).map_err(WriteError)?;
        }
        if !is_meso {
            let skip: Vec<u8> = vec![0; 8];
            self.write_bytes(skip.clone()).map_err(WriteError)?;
        }
        self.write_byte(player_drop as i16).map_err(WriteError)?;
        Ok(self)
    }
}
