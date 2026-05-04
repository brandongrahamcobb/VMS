use crate::models::account::model::Account;
use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::model::Character;
use crate::models::{account, character};
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::player_logged_in;
use crate::net::packet::handler::player_logged_in::read::PlayerLoggedInRead;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::state::SharedState;

pub struct PlayerLoggedInHandler;

impl PlayerLoggedInHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let read: PlayerLoggedInRead = player_logged_in::read::read_play_packet(packet)?;
        let acc: Account =
            account::query::get_account_by_char_id(state.clone(), &read.char_id).await?;
        let session_id: i32 = account::query::get_session_id_by_acc_id(state.clone(), &acc.id)
            .await?
            .unwrap();
        let char: Character =
            character::query::get_character_by_id(state.clone(), &read.char_id).await?;
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .update(session_id, |s| s.map_id = Some(char.map_id));
            state
                .sessions
                .get(&session_id)
                .ok_or(SessionError::NotFound(session_id))?
        };
        let channel_id = session
            .channel_id
            .ok_or(SessionError::NoChannelSelected(session_id))?;
        let (regular_equips, cash_equips): (RegularEquipmentSet, CashEquipmentSet) =
            player_logged_in::service::create_equips_on_join(state.clone(), &char).await?;
        let binds: Vec<Keybinding> =
            player_logged_in::service::create_keybindings_on_join(state.clone(), &char).await?;
        let result = complete_play_handler(
            state.clone(),
            &session_id,
            &char,
            &channel_id,
            &regular_equips,
            &cash_equips,
            binds.clone(),
        )
        .await?;
        Ok(result)
    }
}

async fn complete_play_handler(
    state: SharedState,
    session_id: &i32,
    char: &Character,
    channel_id: &i8,
    regular_equips: &RegularEquipmentSet,
    cash_equips: &CashEquipmentSet,
    binds: Vec<Keybinding>,
) -> Result<HandlerResult<ChannelAction>, NetworkError> {
    let mut result: HandlerResult<ChannelAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_player_logged_in_handler_keymap_packet(&binds)?
        .finish();
    result.add_action(ChannelAction::SendPacket {
        packet: packet.clone(),
    });
    let packet: Packet = Packet::new_empty()
        .build_player_logged_in_handler_char_packet(
            state.clone(),
            char,
            channel_id,
            regular_equips,
            cash_equips,
        )
        .await?
        .finish();
    result.add_action(ChannelAction::SendPacket {
        packet: packet.clone(),
    });
    result.add_action(ChannelAction::Connect {
        session_id: *session_id,
    });
    let session = {
        let state = state.lock().await;
        state
            .sessions
            .get(session_id)
            .ok_or(SessionError::NotFound(*session_id))?
    };
    let packet: Packet = Packet::new_empty()
        .build_spawn_char_packet(state.clone(), &char, &regular_equips, &cash_equips)
        .await?
        .finish();
    result.add_action(ChannelAction::BroadcastPacket {
        session: session.clone(),
        packet: packet.clone(),
    });
    Ok(result)
}
