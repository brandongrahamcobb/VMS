use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::move_player::reader::MovePlayerReader;
use crate::net::packet::handler::move_player::store::MovePlayerStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct MovePlayerHandler;

impl MovePlayerHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: MovePlayerReader = MovePlayerReader::read_move_player_packet(packet)?;
        let store: MovePlayerStore =
            MovePlayerStore::store_move_player(state, session, reader.clone())?;
        let result: HandlerResult = self.build_move_player_result(store.clone())?;
        Ok(result)
    }

    fn build_move_player_result(
        &self,
        store: MovePlayerStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result = HandlerResult::new();
        if !store.too_short && !store.empty {
            let packet: Packet = Packet::new_empty()
                .build_player_move_handler_packet(
                    store.char_model.id,
                    store.movement_bytes.clone(),
                )?
                .finish();
            result.add_action(Action::Send {
                packet: packet.clone(),
                scope: Scope::Local,
            });
            result.add_action(Action::Send {
                packet: packet.clone(),
                scope: Scope::Map,
            });
        }
        Ok(result)
    }
}
