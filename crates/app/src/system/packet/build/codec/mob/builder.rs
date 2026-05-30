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

use crate::system::packet::build::error::PacketBuildError;
use entity::map::model::Point;
use entity::mob::model::{MobMovement, MobWzLife};
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_spawn_mob_packet(
    mob_id: u32,
    mob_life: &MobWzLife,
) -> Result<&mut Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::SpawnMob as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(mob_id as i32).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 1];
    packet.write_bytes(skip).map_err(WriteError)?;
    packet.write_int(mob_life.wz).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 22];
    packet.write_bytes(skip).map_err(WriteError)?;
    packet.write_short(mob_life.x).map_err(WriteError)?;
    packet.write_short(mob_life.y).map_err(WriteError)?;
    let stance: i16 = 0; // 0 not sure
    packet.write_byte(stance).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 2];
    packet.write_bytes(skip).map_err(WriteError)?;
    packet.write_short(mob_life.fh as i16).map_err(WriteError)?;
    let effect: i16 = 0; // 0 = none
    packet.write_byte(effect).map_err(WriteError)?;
    let team: i16 = -1; // -1 = no team
    packet.write_byte(team).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 4];
    packet.write_bytes(skip).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_mob_damage_show_hp_packet(
    mob_id: u32,
    hp_percent: i16,
) -> Result<&mut Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ShowMobHp as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(mob_id as i32).map_err(WriteError)?;
    packet.write_byte(hp_percent).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_kill_mob_packet(mob_id: u32) -> Result<&mut packet, PacketBuildError> {
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
) -> Result<&mut Packet, PacketBuildError> {
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
    mob_id: u32,
    mode: i8,
    wz_id: i32,
    stance: i8,
    fh: u16,
    effect: i8,
    pos: &Point,
    team: i8,
) -> Result<&mut Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::SpawnMobController as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(mode as i16).map_err(WriteError)?;
    packet.write_int(mob_id as i32).map_err(WriteError)?;
    if mode != 0 {
        packet.write_byte(1).map_err(WriteError)?; // skip
        packet.write_int(wz_id).map_err(WriteError)?;
        // 22 bytes skip in client read
        packet.write_bytes(vec![0u8; 22]).map_err(WriteError)?;
        packet.write_short(pos.x).map_err(WriteError)?;
        packet.write_short(pos.y).map_err(WriteError)?;
        packet.write_byte(stance as i16).map_err(WriteError)?;
        packet.write_bytes(vec![0u8; 2]).map_err(WriteError)?; // skip 2
        packet.write_short(fh as i16).map_err(WriteError)?;
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
