use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::models::character::model::Character;
use crate::models::error::ModelError;
use crate::models::{account, character, world};
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct CharListHandler;

impl CharListHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut reader = Cursor::new(packet.bytes);
        reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let world_id = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let channel_id = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let mut acc = account::query::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        acc.selected_world_id = Some(world_id as i16);
        acc.selected_channel_id = Some(channel_id as i16);
        account::query::update(state.clone(), &acc)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let chars = character::query::get_characters_by_account_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let char_max = world::query::get_character_max_by_account_and_world_id(
            state.clone(),
            acc_id,
            world_id as i16,
        )
        .await
        .map_err(DatabaseError::from)
        .map_err(NetworkError::from)
        .unwrap_or(8);
        let use_pic = settings::get_pic_required()?;
        let pic_status = if use_pic { 2 } else { 0 };
        let mut result = HandlerResult::new();
        let packet = build_char_list(chars, char_max, pic_status)?;
        let action = LoginAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}

pub fn build_char_list(
    chars: Vec<Character>,
    char_max: i32,
    pic_status: i8,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::CharList as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0) // account status
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(chars.len() as u8) // number of chars
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    for character in chars {
        character::service::write_list_char(&mut packet, &character)
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
    }
    packet
        .write_byte(pic_status as u8) // use pic?
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(char_max) // Number of character slots
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
