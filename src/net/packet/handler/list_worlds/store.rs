use crate::models::world;
use crate::models::world::model::World;
use crate::net::error::NetworkError;

pub struct ListWorldsStore {
    pub worlds: Vec<World>,
}

impl ListWorldsStore {
    pub fn new() -> Self {
        Self
    }

    pub fn store_list_worlds(&self) -> Result<Self, NetworkError> {
        let worlds = world::service::load_worlds()?;
        Ok(Self {
            worlds: worlds.clon(),
        })
    }
}
