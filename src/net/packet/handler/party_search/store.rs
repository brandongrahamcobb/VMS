use crate::net::error::NetworkError;
use crate::net::packet::handler::party_search::reader::PartySearchReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct PartySearchStore;

impl PartySearchStore {
    pub fn store_party_search(
        state: &SharedState,
        session: Session,
        reader: PartySearchReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(state);
        std::hint::black_box(session);
        std::hint::black_box(reader.clone());
        Ok(Self)
    }
}
