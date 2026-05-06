use crate::net::action::PlayerAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::party_search::reader::PartySearchReader;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct PartySearchHandler;

impl PartySearchHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, NetworkError> {
        let reader: PartySearchReader =
            PartySearchReader::new().read_party_search_packet(packet)?;
        let store: PartySearchStore =
            PartySearchStore::new().store_party_search(state, session, &read)?;
        let result: HandlerResult = self.build_party_search_result(&store)?;
        Ok(result)
    }

    fn build_party_search_result(
        &self,
        store: &PartySearchStore,
    ) -> Result<HandlerResult, NetworkError> {
        // Not implemented
        let mut result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
