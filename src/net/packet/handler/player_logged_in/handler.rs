use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::net::packet::handler::player_logged_in::store::PlayerLoggedInStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct PlayerLoggedInHandler;

impl PlayerLoggedInHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: PlayerLoggedInReader =
            PlayerLoggedInReader::read_player_logged_in_packet(packet)?;
        let store: PlayerLoggedInStore =
            PlayerLoggedInStore::store_player_logged_in(state, session.clone(), reader.clone())
                .await?;
        let result: HandlerResult = self.build_player_logged_in_result(store.clone())?;
        Ok(result)
    }

    fn build_player_logged_in_result(
        &self,
        store: PlayerLoggedInStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_player_logged_in_handler_keymap_packet(store.binds.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_player_logged_in_handler_char_packet(store.char.clone(), store.channel.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        result.add_action(Action::Set(SetAction::SetMap {
            map: store.char.map.clone(),
            scope: Scope::Local,
        }))?;
        let packet: Packet = Packet::new_empty()
            .build_spawn_player_packet(store.char.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        })?;
        for session in store.sessions {
            let packet: Packet = Packet::new_empty()
                .build_spawn_player_packet(session.get_char()?.clone())?
                .finish();
            result.add_action(Action::Send {
                packet: packet.clone(),
                scope: Scope::Local,
            })?;
        }

        Ok(result)
    }
}
