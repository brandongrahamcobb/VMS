use crate::models::world;
use crate::models::world::model::World;
use crate::net::error::NetworkError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ServerStatusStore {
    pub worlds: Vec<World>,
    pub status: u8,
}

impl ServerStatusStore {
    pub fn new() -> Self {
        Self
    }

    pub fn store_server_status(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &ServerStatusReader,
    ) -> Result<Self, NetworkError> {
        let worlds = world::service::load_worlds()?;
        let status: u8 = if worlds.iter().any(|world| !world.channels.is_empty()) {
            0
        } else {
            2
        };
        Ok(Self {
            worlds: worlds.clone(),
            status: status.clone(),
        })
    }
}
