use crate::config::settings;
use crate::constants::WORLDS;
use crate::models::channel;
use crate::models::channel::model::Channel;
use crate::models::error::ModelError;
use crate::models::world::error::WorldError;
use crate::models::world::model::{World, WorldModel};
use crate::runtime::state::SharedState;

pub fn load_worlds() -> Result<Vec<World>, ModelError> {
    let mut worlds: Vec<World> = Vec::new();
    let flag: i16 = settings::get_world_flag()?;
    let event_message: String = settings::get_world_event_message()?;
    let pairs: Vec<(i16, i16)> = settings::get_channel_world_pairs()?;
    for (id, count) in pairs {
        let world_name = name_for_world_by_id(id)
            .ok_or(WorldError::NotFound(id))
            .map_err(ModelError::from)?;
        let world_port = get_world_port_by_id(id)
            .ok_or(WorldError::NotFound(id))
            .map_err(ModelError::from)?;
        let channels =
            channel::service::load_channels(count, world_port).map_err(ModelError::from)?;
        let world_model = WorldModel {
            id,
            name: world_name.to_string(),
            flag,
            event_message: event_message.clone(),
        };
        worlds.push(World {
            model: world_model,
            channels,
        })
    }
    Ok(worlds)
}

pub fn get_world_port_by_id(id: i16) -> Option<i16> {
    WORLDS.get(id as usize).map(|w| w.port)
}

pub fn name_for_world_by_id(id: i16) -> Option<&'static str> {
    WORLDS.get(id as usize).map(|w| w.name)
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

impl World {
    pub fn new() -> Self {
        Self {
            model: WorldModel::new(),
            channels: Vec::<Channel>::new(),
        }
    }
}

impl WorldModel {
    pub fn new() -> Self {
        Self {
            id: -1,
            name: String::new(),
            flag: -1,
            event_message: String::new(),
        }
    }
}
