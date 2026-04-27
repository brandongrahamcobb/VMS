use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::world;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::state::SharedState;

pub struct ServerStatusHandler;

impl ServerStatusHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        _state: SharedState,
        _packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let worlds = world::core::load_worlds()?;
        let status: i8 = if worlds.iter().any(|world| !world.channels.is_empty()) {
            0
        } else {
            2
        };
        let mut result = HandlerResult::new();
        let packet = build_server_status_packet(status)?;
        let action = Action::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
pub fn build_server_status_packet(status: i8) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::ServerStatus as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(status as i16) // Highly populated status!
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
