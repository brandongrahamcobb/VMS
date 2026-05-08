use crate::net::error::NetworkError;
use crate::net::packet::handler::player_map_transfer::reader::PlayerMapTransferReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct PlayerMapTransferStore;

impl PlayerMapTransferStore {
    pub async fn store_player_map_transfer(
        state: &SharedState,
        session: Session,
        reader: PlayerMapTransferReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(state);
        std::hint::black_box(session);
        std::hint::black_box(reader.clone());
        Ok(Self)
    }
}
