use crate::db::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::login::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::relay::RuntimeContext;
use std::io::BufReader;

pub struct CheckCharNameHandler;

impl CheckCharNameHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut reader = BufReader::new(&**packet);
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
        let exists = character::service::get_character_by_name(ctx, &ign).is_ok();
        let mut result = HandlerResult::new();
        let action = LoginAction::CheckCharName { exists, ign };
        result.add_action(action)?;
        Ok(result)
    }
}

pub fn build_check_char_name(name: &str, exists: bool) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::CharNameResponse as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_str_with_length(name)
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
