use crate::models::channel::model::Channel;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::models::{channel, character, map, world};
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct DespawnPlayerStore {
    pub char: Character,
    pub world: World,
    pub channel: Channel,
    pub map: Map,
}

impl DespawnPlayerStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_despawn_player(
        &self,
        state: &SharedState,
        session: &Session,
        read: &DespawnPlayerRead,
    ) -> Result<Self, NetworkError> {
        let world_id = session
            .world_id
            .ok_or(SessionError::NotWorld(*session.id))?;
        let world = world::service::get_world_by_id(&world_id)?;
        let channel_id = session
            .channel_id
            .ok_or(SessionError::NoChannel(*session.id))?;
        let channel = channel::service::get_channel_by_ids(&channel_id, &world_id)?;
        let map_id = session.map_id.ok_or(SessionError::NoMap(*session.id))?;
        let map = map::service::get_map_by_id(&map_id)?;
        let char = character::query::get_character_by_id(state, &read.char_id).await?;
        Ok(Self {
            char: char.clone(),
            world: world.clone(),
            channel: channel.clone(),
            map: map.clone(),
        })
    }
}
