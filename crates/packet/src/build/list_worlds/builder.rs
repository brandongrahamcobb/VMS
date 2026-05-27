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

use std::collections::HashMap;

use crate::build::error::PacketBuildError;
use crate::io::error::IOError::WriteError;
use crate::model::Packet;
use crate::prelude::*;
use config::settings;
use entity::world::wrapper::World;
use op::send::SendOpcode;

impl Packet {
    pub fn build_list_worlds_handler_last_connected_world_packet(
        &mut self,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::LastConnectedWorld as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_list_worlds_handler_recommended_worlds_packet(
        &mut self,
        worlds: &HashMap<i16, World>,
    ) -> Result<&mut Self, PacketBuildError> {
        let recommended_world_names = settings::get_recommended_worlds()?;
        let op = SendOpcode::RecommendedWorlds as i16;
        self.write_short(op).map_err(WriteError)?;
        let count: i16 = recommended_world_names.len() as i16;
        if count != 0 {
            self.write_byte(0).map_err(WriteError)?;
            self.write_byte(count).map_err(WriteError)?;
            for (id, world) in worlds.iter() {
                for world_name in recommended_world_names.clone() {
                    if world.model.name == world_name.clone() {
                        self.write_int(*id as i32).map_err(WriteError)?;
                        self.write_str(world_name).map_err(WriteError)?;
                        self.write_int(0).map_err(WriteError)?;
                    }
                }
            }
        } else {
            self.write_byte(1).map_err(WriteError)?;
        }
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }
}
pub fn build_list_worlds_handler_servers_packets(
    worlds: &HashMap<i16, World>,
) -> Result<Vec<Packet>, PacketBuildError> {
    let mut packets: Vec<Packet> = Vec::new();
    for (world_id, world) in worlds.iter() {
        let mut packet: Packet = Packet::new_empty();
        let op = SendOpcode::ServerList as i16;
        packet.write_short(op).map_err(WriteError)?;
        packet.write_byte(*world_id).map_err(WriteError)?;
        packet
            .write_str_with_length(world.model.name.to_string())
            .map_err(WriteError)?;
        packet.write_byte(world.model.flag).map_err(WriteError)?;
        packet
            .write_str_with_length(world.model.event_message.to_string())
            .map_err(WriteError)?;
        packet.write_byte(100).map_err(WriteError)?;
        packet.write_byte(0).map_err(WriteError)?;
        packet.write_byte(100).map_err(WriteError)?;
        packet.write_byte(0).map_err(WriteError)?;
        packet.write_byte(0).map_err(WriteError)?;
        let channels_length = world.channels.len() as i16;
        packet.write_byte(channels_length).map_err(WriteError)?;
        for (channel_id, channel) in world.channels.iter() {
            let channel_name = String::from("Placeholder");
            packet
                .write_str_with_length(channel_name)
                .map_err(WriteError)?;
            let channel_capacity = channel.model.capacity as i32;
            packet.write_int(channel_capacity).map_err(WriteError)?;
            packet.write_byte(1).map_err(WriteError)?;
            packet.write_byte(*channel_id as i16).map_err(WriteError)?;
            packet.write_byte(*world_id).map_err(WriteError)?;
        }
        packet.write_short(0).map_err(WriteError)?;
        packets.push(packet.finish());
    }
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ServerList as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(0xFF).map_err(WriteError)?;
    packets.push(packet.finish());
    Ok(packets)
}
