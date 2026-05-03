use crate::db::error::DatabaseError;
use crate::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::packet::Packet;
use crate::prelude::*;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct PlayerLoggedInHandler;

impl PlayerLoggedInHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        _session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let mut pkt_reader = Cursor::new(packet.bytes);
        pkt_reader
            .read_short() // prune opcode
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let char_id = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let channel_id = pkt_reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let char = character::query::get_character_by_id(state.clone(), char_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let regular_equips =
            character::equipment_set::query::get_regular_equipment_set_by_character_id(
                state.clone(),
                char_id,
            )
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let cash_equips = character::equipment_set::query::get_cash_equipment_set_by_character_id(
            state.clone(),
            char_id,
        )
        .await
        .map_err(DatabaseError::from)
        .map_err(NetworkError::from)?;
        let binds =
            character::keybinding::query::get_keybindings_by_character_id(state.clone(), char_id)
                .await
                .map_err(DatabaseError::from)
                .map_err(NetworkError::from)?;
        let binds = character::keybinding::service::normalize_keybindings(binds, char.id);
        let mut result: HandlerResult<ChannelAction> = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_play_handler_keymap_packet(&binds)?
            .finish();
        let action = ChannelAction::SendPacket { packet };
        result.add_action(action)?;
        let packet: Packet = Packet::new_empty()
            .build_play_handler_char_packet(
                state.clone(),
                &char,
                channel_id as i16,
                &regular_equips,
                &cash_equips,
            )
            .await?
            .finish();
        let action = ChannelAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
