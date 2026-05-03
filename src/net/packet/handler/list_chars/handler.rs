use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::models::{account, character, world};
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

pub struct CharListHandler;

impl CharListHandler {
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
        let world_id = pkt_reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let channel_id = pkt_reader
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
        let chars = character::query::get_characters_by_account_id_and_world_id(
            state.clone(),
            acc_id,
            world_id as i16,
        )
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
        let mut pic_status = 0;
        let use_pic = settings::get_pic_required()?;
        if let Some(_pic) = acc.pic {
            pic_status = if use_pic { 1 } else { 2 };
        }
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_list_chars_handler_packet(state.clone(), channel_id, chars, char_max, pic_status)
            .await?
            .finish();
        let action = LoginAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
