/* enter_cash_shop/builder.rs
 * The purpose of this module is to build an outgoing cash shop entrance packet.
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

use std::collections::HashMap;

use crate::component::character::MapleCharacter;
use crate::component::item::MapleItem;
use crate::system::packet::build::codec;
use crate::system::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_enter_cash_shop_packet(
    username: String,
    char: &MapleCharacter,
    equips_map: HashMap<i32, Vec<MapleItem>>,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::SetCashShop as i16;
    packet.write_short(op).map_err(WriteError)?;
    // Timestamp / Session Dummy value
    packet.write_long(0).map_err(WriteError)?;
    // Flag
    packet.write_byte(0).map_err(WriteError)?;
    codec::player::builder::build_player_logged_in_meta_part_packet(&mut packet, char, equips_map)?;
    build_cash_shop_meta(&mut packet, username.clone())?;
    Ok(packet)
}

fn build_cash_shop_meta(packet: &mut Packet, username: String) -> Result<(), PacketBuildError> {
    // Dummy values
    // Not MTS
    packet.write_byte(0).map_err(WriteError)?;
    // Account name
    packet
        .write_str_with_length(username.clone())
        .map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    // Special cash items
    packet.write_short(0).map_err(WriteError)?;
    for _ in 0..121 {
        packet.write_byte(0).map_err(WriteError)?;
    }
    for _ in 0..240 {
        packet.write_int(0).map_err(WriteError)?;
    }
    packet.write_int(0).map_err(WriteError)?;
    packet.write_short(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    Ok(())
}
