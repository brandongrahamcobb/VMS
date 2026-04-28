use crate::db::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct CheckCharNameHandler;

impl CheckCharNameHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        _session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let mut reader = Cursor::new(packet.bytes);
        reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let ign = reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let exists = character::service::get_character_by_name(state.clone(), &ign)
            .await
            .is_ok();
        let mut result = HandlerResult::new();
        let packet = build_check_char_name(exists, &ign)?;
        let action = Action::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}

pub fn build_check_char_name(exists: bool, ign: &str) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::CharNameResponse as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_str_with_length(ign)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(exists as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
