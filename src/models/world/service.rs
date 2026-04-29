use crate::config::settings;
use crate::constants::WORLDS;
use crate::models::channel;
use crate::models::error::ModelError;
use crate::models::world::error::WorldError;
use crate::models::world::model::World;

pub fn load_worlds() -> Result<Vec<World>, ModelError> {
    let mut worlds: Vec<World> = Vec::new();
    let flag: i8 = settings::get_world_flag()?;
    let event_message: String = settings::get_world_event_message()?;
    let pairs: Vec<(i16, i16)> = settings::get_channel_world_pairs()?;
    for (id, count) in pairs {
        let world_name = name_for_world_by_id(&id)
            .ok_or(WorldError::NotFound(id))
            .map_err(ModelError::from)?;
        let world_port = get_world_port_by_id(&id)
            .ok_or(WorldError::NotFound(id))
            .map_err(ModelError::from)?;
        let channels =
            channel::service::load_channels(count, id, world_port).map_err(ModelError::from)?;
        worlds.push(World {
            id,
            channels,
            name: world_name.to_string(),
            flag,
            event_message: event_message.clone(),
        })
    }
    Ok(worlds)
}

pub fn get_world_port_by_id(id: &i16) -> Option<i16> {
    WORLDS.get(*id as usize).map(|w| w.port)
}

pub fn name_for_world_by_id(id: &i16) -> Option<&'static str> {
    WORLDS.get(*id as usize).map(|w| w.name)
}
