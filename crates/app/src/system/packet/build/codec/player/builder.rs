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
use crate::component::map::MapleMap;
use crate::system::packet::build::error::PacketBuildError;
use bevy::ecs::hierarchy::ChildOf;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_spawn_player_packet(
    char: &MapleCharacter,
    equips: Vec<(&MapleItem, &ChildOf)>,
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
    let skip: Vec<u8> = vec![0u8; 18];
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
    build_look_meta_part_packet(&mut packet, char, equips)?;
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

pub fn build_set_field_packet(
    char: &MapleCharacter,
    equips: Vec<(&MapleItem, &ChildOf)>,
    channel_id: u8,
    map_wz: i32,
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
    build_player_logged_in_meta_part_packet(&mut packet, char, equips, map_wz)?;
    Ok(packet)
}

pub fn build_look_cash_equipment_part_packet(
    packet: &mut Packet,
    equips: Vec<(&MapleItem, &ChildOf)>,
) -> Result<(), PacketBuildError> {
    for (equip, _) in equips {
        if let Some(ipos) = equip.ipos {
            if equip.base.cash {
                packet.write_byte(ipos).map_err(WriteError)?;
                packet.write_int(equip.base.wz).map_err(WriteError)?;
            }
        } else {
            continue;
        };
    }
    Ok(())
}

pub fn build_look_regular_equipment_part_packet(
    packet: &mut Packet,
    equips: Vec<(&MapleItem, &ChildOf)>,
) -> Result<(), PacketBuildError> {
    for (equip, _) in equips {
        if let Some(ipos) = equip.ipos {
            if !equip.base.cash {
                packet.write_byte(ipos).map_err(WriteError)?;
                packet.write_int(equip.base.wz).map_err(WriteError)?;
            }
        } else {
            continue;
        };
    }
    Ok(())
}

pub fn build_list_char_meta_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    map: &MapleMap,
) -> Result<(), PacketBuildError> {
    packet.write_int(char.id).map_err(WriteError)?;
    packet.write_str(char.ign.clone()).map_err(WriteError)?;
    packet
        .write_bytes(vec![0u8; 13 - char.ign.len()])
        .map_err(WriteError)?;
    packet.write_byte(char.gender_wz).map_err(WriteError)?;
    packet.write_byte(char.skin_wz as i16).map_err(WriteError)?;
    packet.write_int(char.face_wz).map_err(WriteError)?;
    packet.write_int(char.hair_wz).map_err(WriteError)?;
    // Pets... Not implemented yet
    packet.write_long(0).map_err(WriteError)?;
    packet.write_long(0).map_err(WriteError)?;
    packet.write_long(0).map_err(WriteError)?;
    packet.write_byte(char.level).map_err(WriteError)?;
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
    packet.write_int(map.base.wz).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    Ok(())
}

pub fn build_look_meta_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    equips: Vec<(&MapleItem, &ChildOf)>,
) -> Result<(), PacketBuildError> {
    packet.write_byte(char.gender_wz).map_err(WriteError)?;
    packet.write_byte(char.skin_wz as i16).map_err(WriteError)?;
    packet.write_int(char.face_wz).map_err(WriteError)?;
    packet
        .write_byte(0) // megaphone
        .map_err(WriteError)?;
    packet.write_int(char.hair_wz).map_err(WriteError)?;
    build_look_regular_equipment_part_packet(packet, equips.clone())?;
    packet.write_byte(0xFF).map_err(WriteError)?;
    build_look_cash_equipment_part_packet(packet, equips)?;
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

pub fn build_player_logged_in_meta_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    equips: Vec<(&MapleItem, &ChildOf)>,
    map_wz: i32,
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
    packet.write_int(map_wz).map_err(WriteError)?;
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
    equips: Vec<(&MapleItem, &ChildOf)>,
) -> Result<(), PacketBuildError> {
    for (equip, _) in equips.iter() {
        if let Some(ipos) = equip.ipos {
            if !equip.base.cash {
                packet.write_short(-ipos).map_err(WriteError)?;
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
    equips: Vec<(&MapleItem, &ChildOf)>,
) -> Result<(), PacketBuildError> {
    packet.write_int(char.meso).map_err(WriteError)?;
    // Dummy values
    // Inventory slot Capacities
    packet.write_bytes(vec![8u8; 5]).map_err(WriteError)?;
    // Time?
    packet.write_long(0).map_err(WriteError)?;
    build_inventory_regular_equipment_part_packet(packet, equips.clone())?;
    build_look_cash_equipment_part_packet(packet, equips)?;
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

pub fn build_set_exp_packet(exp: i32) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ChangeStats as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(0i16).map_err(WriteError)?; // itemreaction
    packet.write_int(0x10000i32).map_err(WriteError)?; // updatemask: HP
    packet.write_int(exp).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_set_level_packet(level: i16) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ChangeStats as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(0i16).map_err(WriteError)?; // itemreaction
    packet.write_int(0x10i32).map_err(WriteError)?; // updatemask: HP
    packet.write_byte(level).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_level_up_effect_packet(char_id: i32) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ShowForeignEffect as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(char_id).map_err(WriteError)?;
    packet.write_byte(1).map_err(WriteError)?; // level up
    packet.write_int(0).map_err(WriteError)?; // skillid
    packet.write_byte(0).map_err(WriteError)?; // direction
    Ok(packet)
}

pub fn build_set_ap_packet(ap: i16) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ChangeStats as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(0i16).map_err(WriteError)?; // itemreaction
    packet.write_int(0x4000i32).map_err(WriteError)?; // updatemask: HP
    packet.write_short(ap).map_err(WriteError)?;
    Ok(packet)
}

// 0x1       SKIN short
// 0x2       FACE none
// 0x4       HAIR int
// 0x10      LEVEL byte
// 0x20      JOB short
// 0x40      STR short
// 0x80      DEX short
// 0x100     INT short
// 0x200     LUK short
// 0x400     HP short
// 0x800     MAXHP short
// 0x1000    MP short
// 0x2000    MAXMP short
// 0x4000    AP short
// 0x8000    SP short
// 0x10000   EXP int
// 0x20000   FAME short
// 0x40000   MESO int
// 0x180008  PET short
// 0x200000  GACHAEXP short
pub fn build_despawn_player_packet(char_id: i32) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::DespawnPlayer as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(char_id).map_err(WriteError)?;
    Ok(packet)
}
