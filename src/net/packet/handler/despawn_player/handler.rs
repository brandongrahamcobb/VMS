use crate::net::action::{Action, PlayerAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::despawn_player;
use crate::net::packet::handler::despawn_player::read::DespawnPlayerReader;
use crate::net::packet::handler::despawn_player::reader::DespawnPlayerReader;
use crate::net::packet::handler::despawn_player::store::DespawnPlayerStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct DespawnPlayerHandler;

impl DespawnPlayerHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: DespawnPlayerReader =
            DespawnPlayerReader::new().read_despawn_player_handler_packet(packet)?;
        let store: DespawnPlayerStore = DespawnPlayerStore::new()
            .store_despawn_player(state, session, &reader)
            .await?;
        let result: HandlerResult = self.build_despawn_player_result(&store)?;
        Ok(result)
    }

    fn build_despawn_player_result(
        &self,
        store: &DespawnPlayerStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(&store.char.id)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map,
        })?;
    }
}
