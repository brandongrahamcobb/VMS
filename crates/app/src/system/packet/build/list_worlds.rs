/* list_worlds/builder.rs
 * The purpose of this module is to build an outgoing world listing packet.
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

use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::Query;
use config::settings;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

use crate::component::channel::MapleChannel;
use crate::component::world::MapleWorld;
use crate::system::packet::build::error::PacketBuildError;

pub fn build_last_connected_world_packet() -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::LastConnectedWorld as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_recommended_worlds_packet(
    worlds: &Query<(Entity, &MapleWorld)>,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let recommended_world_names = settings::get_recommended_worlds()?;
    let op = SendOpcode::RecommendedWorlds as i16;
    packet.write_short(op).map_err(WriteError)?;
    let count: i16 = recommended_world_names.len() as i16;
    if count != 0 {
        packet.write_byte(0).map_err(WriteError)?;
        packet.write_byte(count).map_err(WriteError)?;
        for (_, world) in worlds.iter() {
            for world_name in recommended_world_names.clone() {
                if world.name == world_name.clone() {
                    packet.write_int(world.id as i32).map_err(WriteError)?;
                    packet.write_str(world_name).map_err(WriteError)?;
                    packet.write_int(0).map_err(WriteError)?;
                }
            }
        }
    } else {
        packet.write_byte(1).map_err(WriteError)?;
    }
    packet.write_int(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    Ok(packet)
}
pub fn build_list_worlds_packets(
    worlds: &Query<(Entity, &MapleWorld)>,
    channels: &Query<(&MapleChannel, &ChildOf)>,
) -> Result<Vec<Packet>, PacketBuildError> {
    let mut packets: Vec<Packet> = Vec::new();
    for (world_entity, world) in worlds.iter() {
        let mut packet: Packet = Packet::new_empty();
        let op = SendOpcode::ServerList as i16;
        packet.write_short(op).map_err(WriteError)?;
        packet.write_byte(world.id).map_err(WriteError)?;
        packet
            .write_str_with_length(world.name.to_string())
            .map_err(WriteError)?;
        packet.write_byte(world.flag).map_err(WriteError)?;
        packet
            .write_str_with_length(world.event_message.to_string())
            .map_err(WriteError)?;
        packet.write_byte(100).map_err(WriteError)?;
        packet.write_byte(0).map_err(WriteError)?;
        packet.write_byte(100).map_err(WriteError)?;
        packet.write_byte(0).map_err(WriteError)?;
        packet.write_byte(0).map_err(WriteError)?;
        let channels_length = channels
            .iter()
            .filter(|(_, parent)| parent.0 == world_entity)
            .count() as i16;
        packet.write_byte(channels_length).map_err(WriteError)?;
        for (channel, _) in channels
            .iter()
            .filter(|(_, parent)| parent.0 == world_entity)
        {
            let channel_name = String::from("Placeholder");
            packet
                .write_str_with_length(channel_name)
                .map_err(WriteError)?;
            let channel_capacity = channel.capacity as i32;
            packet.write_int(channel_capacity).map_err(WriteError)?;
            packet.write_byte(1).map_err(WriteError)?;
            packet.write_byte(channel.id as i16).map_err(WriteError)?;
            packet.write_byte(world.id).map_err(WriteError)?;
        }
        packet.write_short(0).map_err(WriteError)?;
        packets.push(packet);
    }
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ServerList as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(0xFF).map_err(WriteError)?;
    packets.push(packet);
    let packet: Packet = build_last_connected_world_packet()?;
    packets.push(packet);
    let packet: Packet = build_recommended_worlds_packet(worlds)?;
    packets.push(packet);
    Ok(packets)
}
