use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::inc::helpers;
use crate::models::account::error::AccountError;
use crate::models::character::model::{CashEquipment, Character, CharacterEquipment};
use crate::models::error::ModelError;
use crate::models::keybinding::model::Keybinding;
use crate::models::{account, channel, world};
use crate::models::{character, keybinding};
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::{ChannelAction, LoginAction};
use crate::net::packet::handler::char_select;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SpwHandler;

impl SpwHandler {
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
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let char_id = reader
            .read_int()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _macs = reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _hwid = reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let pic = reader
            .read_str_with_length()
            .map_err(IOError::ReadError)
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
        acc.pic = Some(pic);
        account::query::update(state.clone(), &acc)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        {
            let state = state.lock().await;
            state.sessions.update(session.id as u32, |session| {
                session.valid_pic = true;
            })
        }
        acc.selected_char_id = Some(char_id);
        account::query::update(state.clone(), &acc)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let addr = settings::get_address()?;
        let octets = helpers::convert_to_ip_array(addr);
        let worlds = world::service::load_worlds()
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let channel = channel::service::resolve_channel(
            acc.selected_channel_id
                .ok_or(AccountError::MissingField)
                .map_err(ModelError::from)
                .map_err(NetworkError::from)?,
            acc.selected_world_id
                .ok_or(AccountError::MissingField)
                .map_err(ModelError::from)
                .map_err(NetworkError::from)?,
            worlds,
        )
        .map_err(ModelError::from)
        .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let packet = char_select::build_channel_redirect(char_id, octets, channel.port)?;
        result.add_action(LoginAction::SendPacket { packet })?;
        result.add_action(LoginAction::CloseConnection)?;
        Ok(result)
    }
}

fn build_spw_result() -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::CheckSpwResult as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0) // failure
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
