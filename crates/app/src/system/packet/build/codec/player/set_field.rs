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

pub fn build_set_field_packet(
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
    channel_id: u8,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::SetField as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(channel_id as i32).map_err(WriteError)?;
    packet //mode 1
        .write_byte(1)
        .map_err(WriteError)?;
    packet //mode 2
        .write_byte(2)
        .map_err(WriteError)?;
    // Skip 23 bytes
    let skip = vec![0u8; 23];
    packet.write_bytes(skip).map_err(WriteError)?;
    packet.write_int(char.id).map_err(WriteError)?;
    packet.write_str(char.ign.clone()).map_err(WriteError)?;
    packet
        .write_bytes(vec![0u8; 13 - char.ign.len()])
        .map_err(WriteError)?;
    let gender_wz = char.gender_wz;
    packet.write_byte(gender_wz).map_err(WriteError)?;
    let skin_wz = char.skin_wz as i16;
    packet.write_byte(skin_wz).map_err(WriteError)?;
    packet.write_int(char.face_wz).map_err(WriteError)?;
    packet.write_int(char.hair_wz).map_err(WriteError)?;
    // Pets... Not implemented yet
    packet.write_long(0).map_err(WriteError)?;
    packet.write_long(0).map_err(WriteError)?;
    packet.write_long(0).map_err(WriteError)?;
    build_player_logged_in_meta_part_packet(&mut packet, char, equips)?;
    Ok(packet)
}

pub fn build_player_logged_in_meta_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
) -> Result<(), PacketBuildError> {
    let level = char.level;
    packet.write_byte(level).map_err(WriteError)?;
    packet.write_short(char.job_wz).map_err(WriteError)?;
    packet.write_short(char.strength).map_err(WriteError)?;
    packet.write_short(char.dexterity).map_err(WriteError)?;
    packet.write_short(char.intelligence).map_err(WriteError)?;
    packet.write_short(char.luck).map_err(WriteError)?;
    packet.write_short(char.hp).map_err(WriteError)?;
    packet.write_short(char.max_hp).map_err(WriteError)?;
    packet.write_short(char.mp).map_err(WriteError)?;
    packet.write_short(char.max_mp).map_err(WriteError)?;
    packet.write_short(char.ap).map_err(WriteError)?;
    // SP
    packet.write_short(0).map_err(WriteError)?;
    packet.write_int(char.exp).map_err(WriteError)?;
    packet.write_short(char.fame).map_err(WriteError)?;
    // Gach xp?
    packet.write_int(0).map_err(WriteError)?;
    packet.write_int(char.spawn_map_wz).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    let bl_capacity = 25;
    packet.write_byte(bl_capacity).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    build_inventory_part_packet(packet, char, equips)?;
    build_skills_part_packet(packet)?;
    build_quests_part_packet(packet)?;
    build_minigames_part_packet(packet)?;
    build_rings_part_packet(packet)?;
    build_teleport_part_packet(packet)?;
    build_codex_part_packet(packet)?;
    build_new_year_cards_part_packet(packet)?;
    build_area_info_part_packet(packet)?;
    Ok(())
}

fn build_skills_part_packet(packet: &mut Packet) -> Result<(), PacketBuildError> {
    // Dummy values
    // No skills!
    packet.write_short(0).map_err(WriteError)?;
    // No no cooldowns!
    packet.write_short(0).map_err(WriteError)?;
    Ok(())
}

fn build_quests_part_packet(packet: &mut Packet) -> Result<(), PacketBuildError> {
    // Dummy values
    let started_quests = 0;
    packet.write_short(started_quests).map_err(WriteError)?;
    let completed_quests = 0;
    packet.write_short(completed_quests).map_err(WriteError)?;
    Ok(())
}

fn build_minigames_part_packet(packet: &mut Packet) -> Result<(), PacketBuildError> {
    // Dummy values
    packet.write_short(0).map_err(WriteError)?;
    Ok(())
}

fn build_rings_part_packet(packet: &mut Packet) -> Result<(), PacketBuildError> {
    // Dummy values
    let num_crush_rings = 0;
    let num_friendship_rings = 0;
    packet.write_short(num_crush_rings).map_err(WriteError)?;
    packet
        .write_short(num_friendship_rings)
        .map_err(WriteError)?;
    // Not married
    packet.write_short(0).map_err(WriteError)?;
    Ok(())
}

fn build_teleport_part_packet(packet: &mut Packet) -> Result<(), PacketBuildError> {
    // Dummy values
    for _ in 0..5 {
        packet.write_int(0).map_err(WriteError)?;
    }
    for _ in 0..10 {
        packet.write_int(0).map_err(WriteError)?;
    }
    Ok(())
}

fn build_codex_part_packet(packet: &mut Packet) -> Result<(), PacketBuildError> {
    // Dummy values
    let codex_cover = 1;
    let num_cards = 0;
    packet.write_int(codex_cover).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_short(num_cards).map_err(WriteError)?;
    Ok(())
}

fn build_new_year_cards_part_packet(packet: &mut Packet) -> Result<(), PacketBuildError> {
    // Dummy values
    let num_cards = 0;
    packet.write_short(num_cards).map_err(WriteError)?;
    Ok(())
}

fn build_area_info_part_packet(packet: &mut Packet) -> Result<(), PacketBuildError> {
    let num_areas = 0;
    packet.write_short(num_areas).map_err(WriteError)?;
    Ok(())
}

pub fn build_inventory_regular_equipment_part_packet(
    packet: &mut Packet,
    equips: &Vec<MapleItem>,
) -> Result<(), PacketBuildError> {
    for equip in equips {
        if let Some(ipos) = equip.ipos {
            if !equip.base.cash {
                packet.write_short(ipos).map_err(WriteError)?;
                build_inventory_regular_equip_meta_part_packet(packet, equip)?;
            }
        } else {
            continue;
        };
    }
    Ok(())
}

fn build_inventory_regular_equip_meta_part_packet(
    packet: &mut Packet,
    equip: &MapleItem,
) -> Result<(), PacketBuildError> {
    // Dummy values
    packet.write_byte(1).map_err(WriteError)?;
    packet.write_int(equip.base.wz).map_err(WriteError)?;
    const NUM_EQUIP_STATS: i16 = 15;
    let is_cash = false as i16;
    packet.write_byte(is_cash).map_err(WriteError)?;
    packet.write_long(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    for _ in 0..NUM_EQUIP_STATS {
        packet.write_short(0).map_err(WriteError)?;
    }
    packet
        .write_str_with_length(String::new())
        .map_err(WriteError)?;
    packet.write_short(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_short(0).map_err(WriteError)?;
    packet.write_short(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    packet.write_long(0).map_err(WriteError)?;
    packet.write_bytes(vec![0u8; 12]).map_err(WriteError)?;
    Ok(())
}

pub fn build_inventory_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
) -> Result<(), PacketBuildError> {
    packet.write_int(char.meso).map_err(WriteError)?;
    // Dummy values
    // Inventory slot Capacities
    packet.write_bytes(vec![8u8; 5]).map_err(WriteError)?;
    // Time?
    packet.write_long(0).map_err(WriteError)?;
    build_inventory_regular_equipment_part_packet(packet, equips)?;
    look::build_cash_equipment_part_packet(packet, equips)?;
    // End of equipment equipped (all id's) MUST BE ENDED WITH A SHORT 0
    packet.write_short(0).map_err(WriteError)?;
    // Start of equipment inventory (negative id's) MUST BE ENDED WITH A SHORT 0
    packet.write_short(0).map_err(WriteError)?;
    // Start of equipment inventory (postive id's)  MUST BE ENDED WITH A SHORT 0
    packet.write_short(0).map_err(WriteError)?;
    // Skip 2 bytes after equips
    let skip = vec![0u8; 2];
    packet.write_bytes(skip).map_err(WriteError)?;
    // Dummy values
    // Start of USE
    packet.write_byte(0).map_err(WriteError)?;
    // Start of SETUP
    packet.write_byte(0).map_err(WriteError)?;
    // Start of ETC
    packet.write_byte(0).map_err(WriteError)?;
    // Start of CASH
    packet.write_byte(0).map_err(WriteError)?;
    Ok(())
}
