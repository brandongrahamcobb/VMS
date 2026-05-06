use crate::net::action::PlayerAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct PlayerMapTransferHandler;

impl PlayerMapTransferHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: PlayerMapTransferReader =
            PlayerMapTransferReader::new().read_player_map_transfer(packet)?;
        let store: PlayerMapTransferStore =
            PlayerMapTransferStore::new().store_player_map_transfer(state, session, &read)?;
        let result: HandlerResult = self.build_player_map_transfer(&store)?;
        Ok(result)
    }

    fn build_player_map_transfer(
        &self,
        store: &PlayerMapTransferStore,
    ) -> Result<HandlerResult, NetworkError> {
        // Not implemented
        let mut result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
