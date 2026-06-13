/* app/src/system/packet/build/codec/spawn/builder.rs
 * The purpose of this module is to build generic character stats packet bytes.
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
use crate::system::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_char_stats_meta_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    map_wz: i32,
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
    packet.write_int(map_wz).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
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
