use crate::net::error::NetworkError;
use crate::net::packet::handler::player_map_transfer::reader::PlayerMapTransferReader;
use crate::net::packet::handler::player_map_transfer::store::PlayerMapTransferStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct PlayerMapTransferHandler;

impl PlayerMapTransferHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: PlayerMapTransferReader =
            PlayerMapTransferReader::read_player_map_transfer_packet(packet)?;
        let store: PlayerMapTransferStore =
            PlayerMapTransferStore::store_player_map_transfer(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_player_map_transfer(store.clone())?;
        Ok(result)
    }

    fn build_player_map_transfer(
        &self,
        store: PlayerMapTransferStore,
    ) -> Result<HandlerResult, NetworkError> {
        // Not implemented
        std::hint::black_box(store);
        let result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
