use crate::config::settings;
use crate::constants::WORLDS;
use crate::models::error::ModelError;
use crate::models::shroom::channel;
use crate::models::shroom::world::error::WorldError;
use crate::models::shroom::world::model::World;
use crate::runtime::state::SharedState;

pub fn load_worlds() -> Result<Vec<World>, ModelError> {
    let mut worlds: Vec<World> = Vec::new();
    let pairs: Vec<(i16, i16)> = settings::get_channel_world_pairs()?;
    for (id, count) in pairs {
        let world = if let Some(world_model) = WORLDS.get(id as usize) {
            world_model.clone()
        } else {
            return Err(ModelError::from(WorldError::NotFound(id)));
        };
        let channels = channel::service::load_channels(count, world.port)?;
        worlds.push(World {
            model: world,
            channels,
        })
    }
    Ok(worlds)
}

pub async fn get_world_by_id(state: &SharedState, world_id: i16) -> Result<World, ModelError> {
    let worlds = {
        let state = state.lock().await;
        state.worlds.clone()
    };
    for world in worlds {
        if world.model.id == world_id {
            return Ok(world.clone());
        }
        return Err(ModelError::from(WorldError::NotFound(world_id)));
    }
    Err(ModelError::from(WorldError::NoWorlds))
}
