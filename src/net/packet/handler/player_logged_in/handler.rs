use crate::models::account::model::Account;
use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::model::Character;
use crate::models::{account, character};
use crate::net::action::model::PlayerAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in;
use crate::net::packet::handler::player_logged_in::read::PlayerLoggedInRead;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct PlayerLoggedInHandler;

impl PlayerLoggedInHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, NetworkError> {
        let read: PlayerLoggedInRead = player_logged_in::read::read_play_packet(packet)?;
        let acc: Account = account::query::get_account_by_char_id(state, &read.char_id).await?;
        let session_id: i32 = account::query::get_session_id_by_acc_id(state, &acc.id)
            .await?
            .unwrap();
        let char: Character = character::query::get_character_by_id(state, &read.char_id).await?;
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(&session_id)
                .ok_or(SessionError::NotFound(session_id))?
        };
        let world_id: i8 = session
            .channel_id
            .ok_or(SessionError::NoWorld(session.id))?;
        let channel_id: i8 = session
            .channel_id
            .ok_or(SessionError::NoChannel(session.id))?;
        let map_id = char.map_id;
        {
            let state = state.lock().await;
            state.create_location(&session, world_id, channel_id, map_id)?;
        }
        let (regular_equips, cash_equips): (RegularEquipmentSet, CashEquipmentSet) =
            player_logged_in::service::create_equips_on_join(state, &char).await?;
        let binds: Vec<Keybinding> =
            player_logged_in::service::create_keybindings_on_join(state, &char).await?;
        let result =
            complete_play_handler(state, &acc, &char, &regular_equips, &cash_equips, &binds)
                .await?;
        Ok(result)
    }
}

async fn complete_play_handler(
    state: &SharedState,
    acc: &Account,
    char: &Character,
    regular_equips: &RegularEquipmentSet,
    cash_equips: &CashEquipmentSet,
    binds: &Vec<Keybinding>,
) -> Result<HandlerResult<PlayerAction>, NetworkError> {
    let mut result: HandlerResult<PlayerAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_player_logged_in_handler_keymap_packet(binds)?
        .finish();
    result.add_action(PlayerAction::SendLocalPacket {
        packet: packet.clone(),
    });
    let packet: Packet = Packet::new_empty()
        .build_player_logged_in_handler_char_packet(
            state,
            char,
            &channel_id,
            regular_equips,
            cash_equips,
        )
        .await?
        .finish();
    result.add_action(PlayerAction::SendLocalPacket {
        packet: packet.clone(),
    });
    result.add_action(PlayerAction::Connect {
        session: session.clone(),
    });
    let packet: Packet = Packet::new_empty()
        .build_spawn_player_packet(state, char, regular_equips, cash_equips)
        .await?
        .finish();
    result.add_action(PlayerAction::OnJoin {
        session: session.clone(),
        packet: packet.clone(),
        target_world_id: session.world_id.clone(),
        target_channel_id: session.channel_id.clone(),
        target_map_id: Some(char.map_id),
    });
    Ok(result)
}
