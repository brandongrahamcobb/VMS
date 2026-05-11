use crate::{
    net::{
        error::NetworkError, packet::handler::player_map_transfer::reader::PlayerMapTransferReader,
    },
    runtime::{session::model::Session, state::SharedState},
};

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
