/* app/src/system/packet/build/create_char.rs
 * The purpose of this module is to build an outgoing character creation packet.
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

use crate::component::character::MapleCharacter;
use crate::component::item::MapleItem;
use crate::system::packet::build::codec;
use crate::system::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_create_char_packet(
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
    map_wz: i32,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::NewChar as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    build_new_character_look_part_packet(&mut packet, char, equips, map_wz)?;
    Ok(packet)
}

fn build_new_character_look_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
    map_wz: i32,
) -> Result<(), PacketBuildError> {
    codec::player::stats::build_char_stats_meta_part_packet(packet, char, map_wz)?;
    build_new_character_look_meta_part_packet(packet, char, equips)?;
    packet.write_byte(0).map_err(WriteError)?;
    // Disable rank.
    packet.write_byte(0).map_err(WriteError)?;
    Ok(())
}

fn build_new_character_look_meta_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
) -> Result<(), PacketBuildError> {
    let gender_wz = char.gender_wz;
    packet.write_byte(gender_wz).map_err(WriteError)?;
    let skin_wz = char.skin_wz as i16;
    packet.write_byte(skin_wz).map_err(WriteError)?;
    packet.write_int(char.face_wz).map_err(WriteError)?;
    packet
        .write_byte(0) // megaphone
        .map_err(WriteError)?;
    packet.write_int(char.hair_wz).map_err(WriteError)?;
    codec::player::look::build_look_regular_equipment_part_packet(packet, equips)?;
    packet.write_byte(0xFF).map_err(WriteError)?;
    codec::player::look::build_cash_equipment_part_packet(packet, equips)?;
    packet.write_byte(0xFF).map_err(WriteError)?;
    packet
        .write_int(0) //maskedequips -111
        .map_err(WriteError)?;
    // Pet stuff...
    packet.write_int(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    Ok(())
}
