use crate::db::error::DatabaseError;
use crate::db::models::account;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError;
use crate::prelude::*;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct TOSHandler;

impl TOSHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let mut reader = Cursor::new(packet.bytes);
        use tracing::debug;
        let value = reader
            .read_short()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        debug!("TOS: {}", value);
        let confirmed = reader
            .read_byte()
            .map_err(IOError::ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        if confirmed != 0x01 {
            return Err(NetworkError::from(PacketError::from(
                HandlerError::LoginError,
            )));
        }
        let acc_id = 1; //placeholder
        let mut acc = account::service::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        acc.accepted_tos = true;
        account::service::update(state.clone(), &acc)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let packet = credentials::build_successful_login_packet(&acc)?;
        let action = Action::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
