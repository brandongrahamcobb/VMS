use crate::db::error::DatabaseError;
use crate::db::models::account;
use crate::net::character::error::CharacterError;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::WorldAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::WriteError;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

const MOVEMENT_HEADER_LEN: usize = 9;

pub struct MovePlayerHandler;

impl MovePlayerHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<WorldAction>, NetworkError> {
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let acc = account::service::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let char_id = acc
            .selected_character_id
            .ok_or(CharacterError::NotSelected(acc_id))
            .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        if packet.bytes.len() <= 2 + MOVEMENT_HEADER_LEN {
            let action = WorldAction::Simple;
            result.add_action(action)?;
            return Ok(result);
        }
        let movement_fragment = &packet.bytes[(2 + MOVEMENT_HEADER_LEN)..];
        if movement_fragment.is_empty() || movement_fragment[0] == 0 {
            let action = WorldAction::Simple;
            result.add_action(action)?;
            return Ok(result);
        }
        let movement_bytes = movement_fragment.to_vec();
        result.add_action(WorldAction::FieldMove {
            movement_bytes: movement_bytes.clone(),
        });
        let packet = build_player_move(char_id, &movement_bytes)?;
        result.add_action(WorldAction::SendPacket { packet });
        Ok(result)
    }
}

pub fn build_player_move(char_id: i32, movement_bytes: &[u8]) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(SendOpcode::MovePlayer as i16)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(char_id)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(movement_bytes)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
