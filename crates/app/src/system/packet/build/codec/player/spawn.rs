/* spawn_player/builder.rs
 * The purpose of this module is to build an outgoing spawn player packet.
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
use crate::system::packet::build::codec::player::look;
use crate::system::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_spawn_player_packet(
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::SpawnPlayer as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(char.id).map_err(WriteError)?;
    let level: i16 = char.level;
    packet.write_byte(level).map_err(WriteError)?;
    packet
        .write_str_with_length(char.ign.clone())
        .map_err(WriteError)?;
    let guild_name = String::from("Guild Name");
    packet
        .write_str_with_length(guild_name)
        .map_err(WriteError)?;
    let skip: Vec<u8> = vec![0u8; 14];
    packet.write_bytes(skip).map_err(WriteError)?;
    let morphed: i32 = 0; // 2 if morphed
    packet.write_int(morphed).map_err(WriteError)?;
    let buff_mask_one: i32 = 0;
    packet.write_int(buff_mask_one).map_err(WriteError)?;
    if buff_mask_one != 0 {
        if morphed == 2 {
            let buff_value = 0; // changes if morphed
            packet.write_short(buff_value).map_err(WriteError)?;
        } else {
            let buff_value = 0; // changes if not morphed
            packet.write_byte(buff_value).map_err(WriteError)?;
        }
    }
    let buff_mask_two: i32 = 0; // 0 not sure
    packet.write_int(buff_mask_two).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0u8; 43];
    packet.write_bytes(skip).map_err(WriteError)?;
    let mount: i32 = 0; // 0 not sure
    packet.write_int(mount).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0u8; 61];
    packet.write_bytes(skip).map_err(WriteError)?;
    packet.write_short(char.job_wz).map_err(WriteError)?;
    look::build_look_meta_part_packet(&mut packet, char, equips)?;
    let count: i32 = 5110000;
    packet.write_int(count).map_err(WriteError)?;
    let item_effect: i32 = 0; // 0 not sure
    packet.write_int(item_effect).map_err(WriteError)?;
    let chair: i32 = 0; // 0 not sure
    packet.write_int(chair).map_err(WriteError)?;
    let position_x: i16 = 0; // 0 this is a point so it might be wrong
    let position_y: i16 = 0; // 0 this is a point so it might be wrong
    packet.write_short(position_x).map_err(WriteError)?;
    packet.write_short(position_y).map_err(WriteError)?;
    let stance: i16 = 0; // 0 not sure
    packet.write_byte(stance).map_err(WriteError)?;
    let skip = vec![0u8; 3];
    packet.write_bytes(skip).map_err(WriteError)?;
    for _ in 0..3 {
        let available = 0; // 0 not sure
        packet.write_byte(available).map_err(WriteError)?;
        if available == 1 {
            let byte_two: i16 = 0; // 0 not sure
            packet.write_byte(byte_two).map_err(WriteError)?;
            let pet_id: i32 = 0; // 0 is definitely not right
            packet.write_int(pet_id).map_err(WriteError)?;
            let pet_name = String::from("George");
            packet.write_str_with_length(pet_name).map_err(WriteError)?;
            let unique_id = 0; // 0 not sure
            packet.write_int(unique_id).map_err(WriteError)?;
            let skip = 0;
            packet.write_int(skip).map_err(WriteError)?;
            packet.write_short(position_x).map_err(WriteError)?;
            packet.write_short(position_y).map_err(WriteError)?;
            packet.write_byte(stance).map_err(WriteError)?;
            let fhid = 0; // 0 not sure
            packet.write_int(fhid).map_err(WriteError)?;
        } else {
            break;
        }
    }
    let mount_level = 0; // 0 not sure
    packet.write_int(mount_level).map_err(WriteError)?;
    let mount_exp = 0; // 0 not sure
    packet.write_int(mount_exp).map_err(WriteError)?;
    let mount_tiredness = 0; // 0 not sure
    packet.write_int(mount_tiredness).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0u8; 1];
    packet.write_bytes(skip).map_err(WriteError)?; // shop stuff
    let chalkboard_bool: bool = false; // false not sure
    let chalkboard: i16 = chalkboard_bool as i16; // false not sure
    packet.write_byte(chalkboard).map_err(WriteError)?;
    if chalkboard_bool {
        let chalkboard_text = String::from("Placeholder");
        packet
            .write_str_with_length(chalkboard_text)
            .map_err(WriteError)?;
    }
    let skip: Vec<u8> = vec![0u8; 3];
    packet.write_bytes(skip).map_err(WriteError)?;
    let team: i16 = 0; // 0 not sure
    packet.write_byte(team).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_despawn_player_packet(char_id: i32) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::DespawnPlayer as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(char_id).map_err(WriteError)?;
    Ok(packet)
}
