use crate::config::settings;
use crate::constants::WORLDS;
use crate::models::shroom::world::model::World;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_list_worlds_handler_last_connected_world_packet(
        &mut self,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::LastConnectedWorld as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_list_worlds_handler_recommended_worlds_packet(
        &mut self,
    ) -> Result<&mut Self, NetworkError> {
        let recommended_world_names = settings::get_recommended_worlds()?;
        let op = SendOpcode::RecommendedWorlds as i16;
        self.write_short(op).map_err(WriteError)?;
        let count: i16 = recommended_world_names.len() as i16;
        if count != 0 {
            self.write_byte(0).map_err(WriteError)?;
            self.write_byte(count).map_err(WriteError)?;
            for world in WORLDS {
                for world_name in recommended_world_names.clone() {
                    if world.name == world_name.clone() {
                        let id = world.id as i32;
                        self.write_int(id).map_err(WriteError)?;
                        self.write_str(world.name.to_string()).map_err(WriteError)?;
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

    pub fn build_list_worlds_handler_servers_packet(
        &mut self,
        worlds: Vec<World>,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ServerList as i16;
        self.write_short(op).map_err(WriteError)?;
        for world in worlds {
            let world_id = world.model.id as i16;
            self.write_byte(world_id).map_err(WriteError)?;
            self.write_str(world.model.name.to_string())
                .map_err(WriteError)?;
            self.write_byte(world.model.flag).map_err(WriteError)?;
            self.write_str(world.model.event_message.to_string())
                .map_err(WriteError)?;
            self.write_byte(100).map_err(WriteError)?;
            self.write_byte(0).map_err(WriteError)?;
            self.write_byte(100).map_err(WriteError)?;
            self.write_byte(0).map_err(WriteError)?;
            self.write_byte(0).map_err(WriteError)?;
            let channels_length = world.channels.len() as i16;
            self.write_byte(channels_length).map_err(WriteError)?;
            for channel in world.channels.clone() {
                let channel_name = String::from("Placeholder");
                self.write_str(channel_name).map_err(WriteError)?;
                let channel_capacity = channel.model.capacity as i32;
                self.write_int(channel_capacity).map_err(WriteError)?;
                self.write_byte(1).map_err(WriteError)?;
                let channel_id = channel.model.id as i16;
                self.write_byte(channel_id).map_err(WriteError)?;
                let world_id = world.model.id as i16;
                self.write_byte(world_id).map_err(WriteError)?;
            }
            self.write_short(0).map_err(WriteError)?;
        }
        self.write_byte(0xFF).map_err(WriteError)?;
        Ok(self)
    }
}
