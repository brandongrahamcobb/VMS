use crate::net::action::model::{Action, PlayerAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::despawn_player;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
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
        let read = DespawnPlayerRead::new().read_despawn_player_handler_packet(packet)?;
        let store = DespawnPlayerStore::new()
            .store_despawn_player(state, &read)
            .await?;
        let result = self.build_despawn_player_result(session, &store)?;
        Ok(result)
    }

    fn build_despawn_player_result(
        &self,
        session: &Session,
        store: &DespawnPlayerStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(&store.char.id)?
            .finish();
        result.add_action(Action::Local {
            packet: packet.clone(),
        });
        result.add_action(Action::Player(PlayerAction::ExitMap {
            session: session.clone(),
            packet: packet.clone(),
            source_world: store.world.clone(),
            source_channel: store.channel.clone(),
            source_map: store.map.clone(),
        }));
    }
}
