use crate::net::error::NetworkError;
use crate::net::packet::handler::player_map_transfer::reader::PlayerMapTransferReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct PlayerMapTransferStore;

impl PlayerMapTransferStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_player_map_transfer(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &PlayerMapTransferReader,
    ) -> Result<Self, NetworkError> {
        debug!("{:?} {:?} {:?}", state, session, reader);
        Ok(Self)
    }
}
