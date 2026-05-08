use crate::models::world::model::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::server_status::reader::ServerStatusReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ServerStatusStore {
    pub worlds: Vec<World>,
    pub status: i8,
}

impl ServerStatusStore {
    pub fn store_server_status(
        state: &SharedState,
        session: Session,
        reader: ServerStatusReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(session);
        std::hint::black_box(reader.clone());
        let worlds = state.worlds.clone();
        let status: i8 = if worlds.iter().any(|world| !world.channels.is_empty()) {
            0
        } else {
            2
        };
        Ok(Self { worlds, status })
    }
}
