/* mob/builder.rs
 * The purpose of this module is to build mob packets.
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

use crate::component::mob::MapleMob;
use crate::system::packet::build::error::PacketBuildError;
use base::mob::MobMovement;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_spawn_mob_packet(
    mob: &MapleMob,
    stance: i8,
    effect: i8,
    team: i8,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::SpawnMob as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(mob.id as i32).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 1];
    packet.write_bytes(skip).map_err(WriteError)?;
    packet.write_int(mob.base.wz).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 22];
    packet.write_bytes(skip).map_err(WriteError)?;
    packet.write_short(mob.base.x).map_err(WriteError)?;
    packet.write_short(mob.base.y).map_err(WriteError)?;
    packet.write_byte(stance as i16).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 2];
    packet.write_bytes(skip).map_err(WriteError)?;
    packet.write_short(mob.base.fh as i16).map_err(WriteError)?;
    packet.write_byte(effect as i16).map_err(WriteError)?;
    packet.write_byte(team as i16).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 4];
    packet.write_bytes(skip).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_mob_damage_show_hp_packet(
    mob_id: u32,
    hp_percent: i16,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ShowMobHp as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(mob_id as i32).map_err(WriteError)?;
    packet.write_byte(hp_percent).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_kill_mob_packet(mob_id: u32) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::KillMob as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(mob_id as i32).map_err(WriteError)?;
    packet.write_byte(1).map_err(WriteError)?; //animation likely from wz, 0 is not correct
    Ok(packet)
}

pub fn build_mob_move_packet(
    mob_id: u32,
    skillb: u8,
    skill0: u8,
    skill1: u8,
    skill2: u8,
    skill3: u8,
    skill4: u8,
    pos_x: i16,
    pos_y: i16,
    movements: Vec<MobMovement>,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::MoveMonster as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(mob_id as i32).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 1];
    packet.write_bytes(skip.clone()).map_err(WriteError)?;
    packet.write_byte(skillb as i16).map_err(WriteError)?;
    packet.write_byte(skill0 as i16).map_err(WriteError)?;
    packet.write_byte(skill1 as i16).map_err(WriteError)?;
    packet.write_byte(skill2 as i16).map_err(WriteError)?;
    packet.write_byte(skill3 as i16).map_err(WriteError)?;
    packet.write_byte(skill4 as i16).map_err(WriteError)?;
    packet.write_short(pos_x).map_err(WriteError)?;
    packet.write_short(pos_y).map_err(WriteError)?;
    packet
        .write_byte(movements.len() as i16)
        .map_err(WriteError)?;
    for m in &movements {
        packet.write_byte(m.command as i16).map_err(WriteError)?;
        match m.command {
            0 | 5 | 17 => {
                packet.write_short(m.x).map_err(WriteError)?;
                packet.write_short(m.y).map_err(WriteError)?;
                packet.write_short(m.last_x).map_err(WriteError)?;
                packet.write_short(m.last_y).map_err(WriteError)?;
                packet.write_short(m.fh as i16).map_err(WriteError)?;
                packet.write_byte(m.new_state as i16).map_err(WriteError)?;
                packet.write_short(m.duration).map_err(WriteError)?;
            }
            1 | 2 | 6 | 12 | 13 | 16 => {
                packet.write_short(m.x).map_err(WriteError)?;
                packet.write_short(m.y).map_err(WriteError)?;
                packet.write_byte(m.new_state as i16).map_err(WriteError)?;
                packet.write_short(m.duration).map_err(WriteError)?;
            }
            _ => {}
        }
    }
    Ok(packet)
}

pub fn build_spawn_mob_controller_packet(
    mob: &MapleMob,
    mode: i8,
    stance: i8,
    effect: i8,
    team: i8,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::SpawnMobController as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(mode as i16).map_err(WriteError)?;
    packet.write_int(mob.id as i32).map_err(WriteError)?;
    if mode != 0 {
        packet.write_byte(1).map_err(WriteError)?; // skip
        packet.write_int(mob.base.wz).map_err(WriteError)?;
        // 22 bytes skip in client read
        packet.write_bytes(vec![0u8; 22]).map_err(WriteError)?;
        packet.write_short(mob.base.x).map_err(WriteError)?;
        packet.write_short(mob.base.y).map_err(WriteError)?;
        packet.write_byte(stance as i16).map_err(WriteError)?;
        packet.write_bytes(vec![0u8; 2]).map_err(WriteError)?; // skip 2
        packet.write_short(mob.base.fh as i16).map_err(WriteError)?;
        packet.write_byte(effect as i16).map_err(WriteError)?;
        if effect > 0 {
            packet.write_byte(0).map_err(WriteError)?;
            packet.write_short(0).map_err(WriteError)?;
        }
        packet.write_byte(team as i16).map_err(WriteError)?;
        packet.write_bytes(vec![0u8; 4]).map_err(WriteError)?; // skip 4
    }
    Ok(packet)
}
