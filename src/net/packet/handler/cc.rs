use crate::config::settings;
use crate::net::channel;
use crate::net::channel::core::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::state::SharedState;
use core::net::IpAddr;
use std::io::Cursor;

pub struct ChangeChannelHandler;

impl ChangeChannelHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        _state: SharedState,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let mut reader = Cursor::new(packet.bytes);
        let _op = reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let channel_id = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _tick = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let world_id = 0; //placeholder
        let channel = channel::core::resolve_channel(channel_id as i16, world_id)?;
        let packet = build_channel_change_packet(&channel)?;
        let action = Action::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}

pub fn build_channel_change_packet(channel: &Channel) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let addr = settings::get_world_server_addr()?;
    let v4 = match addr.ip() {
        IpAddr::V4(v4) => v4,
        IpAddr::V6(_) => return Err(NetworkError::UnexpectedError),
    };
    let op = SendOpcode::ChangeChannel as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(&v4.octets())
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(channel.port)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
