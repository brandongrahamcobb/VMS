use crate::db::error::DatabaseError;
use crate::models::account::model::Account;
use crate::models::character::error::CharacterError;
use crate::models::character::model::Character;
use crate::models::error::ModelError;
use crate::models::{account, character};
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::WriteError;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct EnterCashShopHandler;

impl EnterCashShopHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        _packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let acc = account::query::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let char_id = acc
            .selected_char_id
            .ok_or(CharacterError::NotSelected(acc_id))
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let char = character::query::get_character_by_id(state.clone(), char_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let packet = build_cash_shop_packet(&acc, &char, &session)?;
        let action = ChannelAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
pub fn build_cash_shop_packet(
    acc: &Account,
    char: &Character,
    session: &Session,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::SetCashShop as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Timestamp / Session
    packet
        .write_long(session.id as i64)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Flag
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    character::service::write_game_char(&mut packet, char)
        .map_err(ModelError::from)
        .map_err(NetworkError::from)?;
    write_cash_shop(&mut packet, acc)?;
    Ok(packet)
}

fn write_cash_shop(packet: &mut Packet, acc: &Account) -> Result<(), NetworkError> {
    // Not MTS
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Account name
    packet
        .write_str_with_length(&acc.username)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Special cash items
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    for _ in 0..121 {
        packet
            .write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
    }
    for _ in 0..240 {
        packet
            .write_int(0)
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
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(())
}
