use crate::net::error::NetworkError;
use crate::net::packet::handler::party_search::reader::PartySearchReader;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct PartySearchStore;

impl PartySearchStore {
    pub fn new() -> Self {
        Self
    }

    pub fn store_party_search(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &PartySearchReader,
    ) -> Result<Self, NetworkError> {
        debug!("{:?} {:?} {:?}", state, session, reader);
        Ok(Self)
    }
}
