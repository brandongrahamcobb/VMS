use crate::db::error::DatabaseError;
use crate::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::packet::Packet;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct DeleteCharacterHandler;

impl DeleteCharacterHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut pkt_reader = Cursor::new(packet.bytes);
        pkt_reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _pic = pkt_reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let char_id = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        character::query::delete_character(state.clone(), acc_id, char_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_delete_char_handler_packet(char_id, 0x00)?
            .finish();
        let action = LoginAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
