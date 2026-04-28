use crate::config::settings;
use crate::constants::WORLDS;
use crate::models::channel::model::Channel;
use crate::models::world::error::WorldError;
use crate::models::world::model::World;

pub fn load_worlds() -> Result<Vec<World>, WorldError> {
    let mut worlds: Vec<World> = Vec::new();
    let capacity: i16 = settings::get_channel_capacity()?;
    let flag: i8 = settings::get_world_flag()?;
    let event_message: String = settings::get_world_event_message()?;
    let pairs: Vec<(i16, i16)> = settings::get_channel_world_pairs()?;
    let world_port = settings::get_world_port()?;
    for (id, count) in pairs {
        let name: &str = name_for_world_by_id(id).unwrap_or("Unknown");
        let channels: Vec<Channel> = (0..count)
            .map(|channel_id| Channel {
                capacity: capacity,
                channel_id,
                name: format!("{name}-{}", channel_id + 1),
                port: world_port + 1,
                world_id: id,
            })
            .collect();
        worlds.push(World {
            id,
            name: name.to_string(),
            flag,
            event_message: event_message.clone(),
            channels,
        })
    }
    Ok(worlds)
}

pub fn name_for_world_by_id(id: i16) -> Option<&'static str> {
    WORLDS.get(id as usize).map(|w| w.name)
}
