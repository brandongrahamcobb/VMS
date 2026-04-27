use crate::config::settings;
use crate::constants::WORLDS;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::world;
use crate::net::world::core::World;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::state::SharedState;

pub struct WorldListHandler;

impl WorldListHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        _state: SharedState,
        _packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let mut result = HandlerResult::new();
        let packets = build_world_packets()?;
        for packet in packets {
            let action = Action::SendPacket { packet };
            result.add_action(action)?;
        }
        Ok(result)
    }
}

pub fn build_world_packets() -> Result<Vec<Packet>, NetworkError> {
    let worlds = world::core::load_worlds()?;
    let mut packets: Vec<Packet> = Vec::new();
    packets.push(build_world_list_packet(worlds)?);
    packets.push(build_last_connected_world_packet()?);
    packets.push(build_recommended_worlds_packet()?);
    Ok(packets)
}

fn build_last_connected_world_packet() -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::LastConnectedWorld as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

fn build_recommended_worlds_packet() -> Result<Packet, NetworkError> {
    let recommended_world_names = settings::get_recommended_worlds()?;
    let mut packet = Packet::new_empty();
    let op = SendOpcode::RecommendedWorlds as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    let count: i8 = recommended_world_names.len().try_into().unwrap();
    if count != 0 {
        packet
            .write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(count as u8)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        for world in WORLDS {
            for world_name in &recommended_world_names {
                if world_name == &world.name {
                    let id = world.id;
                    packet
                        .write_int(id as i32)
                        .map_err(WriteError)
                        .map_err(PacketError::from)
                        .map_err(NetworkError::from)?;
                    packet
                        .write_str(world.name)
                        .map_err(WriteError)
                        .map_err(PacketError::from)
                        .map_err(NetworkError::from)?;
                    packet
                        .write_int(0)
                        .map_err(WriteError)
                        .map_err(PacketError::from)
                        .map_err(NetworkError::from)?;
                }
            }
        }
    } else {
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
    }
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

fn build_world_list_packet(worlds: Vec<World>) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::ServerList as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    for world in worlds {
        packet
            .write_byte(world.id as u8)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_str_with_length(world.name.as_str())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(world.flag as u8)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_str_with_length(world.event_message.as_str())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_byte(world.channels.len() as u8)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        for channel in &world.channels {
            packet
                .write_str_with_length(channel.name.as_str())
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
            packet
                .write_int(channel.capacity as i32)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
            packet
                .write_byte(1)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
            packet
                .write_byte(channel.channel_id as u8)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
            packet
                .write_byte(world.id as u8)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
        }
        packet
            .write_short(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
    }
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
