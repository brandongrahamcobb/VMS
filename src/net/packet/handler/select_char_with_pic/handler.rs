use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::inc::helpers;
use crate::models::{account, channel, world};
use crate::models::account::error::AccountError;
use crate::models::error::ModelError;
use crate::net::error::NetworkError;
use crate::net::packet::packet::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::ReadError;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct SelectCharWithPicHandler;

impl SelectCharWithPicHandler {
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
        pkt_reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let char_id = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _macs = pkt_reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _hwid = pkt_reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let pic = pkt_reader
            .read_str_with_length()
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
        acc.selected_char_id = Some(char_id);
        account::query::update(state.clone(), &acc)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let acc_pic = acc.pic
            .ok_or(AccountError::MissingField)
            .map_err(ModelError::from)
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
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        let result = if pic == acc_pic { 
            {
                let state = state.lock().await;
                state.sessions.update(session.id as u32, |session| {
                    session.valid_pic = true;
                })
            }
            let packet: Packet = Packet::new_empty()
                .build_select_char_handler_packet(char_id, octets, channel.port)?
                .finish();
            result.add_action(LoginAction::SendPacket { packet })?;
            result.add_action(LoginAction::CloseConnection)?;
            result
        } else {
            let packet: Packet = Packet::new_empty().build_select_char_handler_failed_pic_packet()?.finish();
            result.add_action(LoginAction::SendPacket { packet })?;
            result
        };
        Ok(result)
    }
}


