use crate::models::shroom::world::model::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_worlds::reader::ListWorldsReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ListWorldsStore {
    pub worlds: Vec<World>,
}

impl ListWorldsStore {
    pub async fn store_list_worlds(
        state: &SharedState,
        session: Session,
        reader: ListWorldsReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(session);
        std::hint::black_box(reader.clone());
        let worlds = {
            let state = state.lock().await;
            state.worlds.clone()
        };
        Ok(Self { worlds })
    }
}
