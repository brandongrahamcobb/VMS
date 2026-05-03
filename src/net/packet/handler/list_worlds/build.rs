use crate::config::settings;
use crate::constants::WORLDS;
use crate::models::world::model::World;
use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::packet::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_list_worlds_handler_last_connected_world_packet(
        &mut self,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::LastConnectedWorld as i16;
        self.write_short(op)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(self)
    }

    pub fn build_list_worlds_handler_recommended_worlds_packet(
        &mut self,
    ) -> Result<&mut Self, NetworkError> {
        let recommended_world_names = settings::get_recommended_worlds()?;
        let op = SendOpcode::RecommendedWorlds as i16;
        self.write_short(op)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let count: i8 = recommended_world_names.len().try_into().unwrap();
        if count != 0 {
            self.write_byte(0)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_byte(count as u8)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            for world in WORLDS {
                for world_name in &recommended_world_names {
                    if world_name == &world.name {
                        let id = world.id;
                        self.write_int(id as i32)
                            .map_err(WriteError)
                            .map_err(PacketError::from)
                            .map_err(NetworkError::from)?;

                        self.write_str(world.name)
                            .map_err(WriteError)
                            .map_err(PacketError::from)
                            .map_err(NetworkError::from)?;

                        self.write_int(0)
                            .map_err(WriteError)
                            .map_err(PacketError::from)
                            .map_err(NetworkError::from)?;
                    }
                }
            }
        } else {
            self.write_byte(1)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
        }
        self.write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(self)
    }

    pub fn build_list_worlds_handler_servers_packet(
        &mut self,
        worlds: Vec<World>,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ServerList as i16;
        self.write_short(op)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        for world in worlds {
            self.write_byte(world.id as u8)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_str_with_length(world.name.as_str())
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_byte(world.flag as u8)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_str_with_length(world.event_message.as_str())
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_byte(100)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_byte(0)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_byte(100)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_byte(0)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_byte(0)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            self.write_byte(world.channels.len() as u8)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;

            for channel in &world.channels {
                self.write_str_with_length("Placeholder Channel Name")
                    .map_err(WriteError)
                    .map_err(PacketError::from)
                    .map_err(NetworkError::from)?;

                self.write_int(channel.capacity as i32)
                    .map_err(WriteError)
                    .map_err(PacketError::from)
                    .map_err(NetworkError::from)?;

                self.write_byte(1)
                    .map_err(WriteError)
                    .map_err(PacketError::from)
                    .map_err(NetworkError::from)?;

                self.write_byte(channel.id as u8)
                    .map_err(WriteError)
                    .map_err(PacketError::from)
                    .map_err(NetworkError::from)?;

                self.write_byte(world.id as u8)
                    .map_err(WriteError)
                    .map_err(PacketError::from)
                    .map_err(NetworkError::from)?;
            }
            self.write_short(0)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
        }
        self.write_byte(0xFF)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(self)
    }
}
