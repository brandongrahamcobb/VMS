use crate::net::error::NetworkError;
use crate::net::packet::handler::party_search::reader::PartySearchReader;
use crate::net::packet::handler::party_search::store::PartySearchStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct PartySearchHandler;

impl PartySearchHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: PartySearchReader = PartySearchReader::read_party_search_packet(packet)?;
        let store: PartySearchStore =
            PartySearchStore::store_party_search(state, session, reader.clone())?;
        let result: HandlerResult = self.build_party_search_result(store.clone())?;
        Ok(result)
    }

    fn build_party_search_result(
        &self,
        store: PartySearchStore,
    ) -> Result<HandlerResult, NetworkError> {
        // Not implemented
        std::hint::black_box(store);
        let result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
