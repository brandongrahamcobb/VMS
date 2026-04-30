use crate::db::error::DatabaseError;
use crate::inc::helpers;
use crate::models::account;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct PicHandler;

impl PicHandler {
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
        let mut result = HandlerResult::new();
        let action = LoginAction::Simple;
        result.add_action(action)?;
        Ok(result)
    }
}

fn build_spw_result(_state: SharedState) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::CheckSpwResult as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1) // 0 for failure, anything else for success
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
