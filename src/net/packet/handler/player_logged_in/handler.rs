use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::net::packet::handler::player_logged_in::store::PlayerLoggedInStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
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
            .build_player_logged_in_handler_keymap_packet(store.bind_models.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        let packet: Packet = Packet::new_empty()
            .build_player_logged_in_handler_char_packet(store.char.clone(), store.channel_model.id)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        let packet: Packet = Packet::new_empty()
            .build_spawn_player_packet(store.char.model.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map,
        });
        Ok(result)
    }
}
