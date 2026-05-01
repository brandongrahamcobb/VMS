use crate::db::error::DatabaseError;
use crate::models::account;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct TOSHandler;

impl TOSHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut pkt_reader = Cursor::new(packet.bytes);
        pkt_reader
            .read_short()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let confirmed = pkt_reader
            .read_byte()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        if confirmed != 0x01 {
            return Err(NetworkError::from(PacketError::from(
                HandlerError::LoginError,
            )));
        }
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let mut acc = account::query::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        acc.accepted_tos = true;
        account::query::update(state.clone(), &acc)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        {
            let state = state.lock().await;
            state.sessions.update(session.id as u32, |session| {
                session.authenticated = true;
            })
        }
        let mut result = HandlerResult::new();
        let packet = credentials::build_successful_login_packet(&acc)?;
        let action = LoginAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
