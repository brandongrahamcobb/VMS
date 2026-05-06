use crate::config::settings;
use crate::models::channel::error::ChannelError;
use crate::models::channel::model::Channel;
use crate::models::error::ModelError;
use crate::models::world;

pub fn get_channel_by_ids(channel_id: &i8, world_id: &i8) -> Result<Channel, ModelError> {
    let worlds = world::service::load_worlds().map_err(ModelError::from)?;
    for world in worlds {
        if world.id == *world_id {
            for channel in &world.channels {
                if channel.id == *channel_id {
                    return Ok(channel.clone());
                }
            }
            return Err(ModelError::from(ChannelError::NotFound(*channel_id)));
        }
    }
    Err(ModelError::from(ChannelError::UnexpectedError))
}

pub fn load_channels(
    channel_count: &i16,
    world_id: &i8,
    world_port: &i16,
) -> Result<Vec<Channel>, ModelError> {
    let mut channels = Vec::new();
    let capacity: i16 = settings::get_channel_capacity()?;
    let flag: i8 = settings::get_world_flag()?;
    let mut id = 0;
    let count = *channel_count;
    for count in 0..count {
        let port = *world_port + 1 + count;
        channels.push(Channel {
            capacity: capacity,
            id,
            flag,
            port,
            world_id: *world_id,
        });
        id += 1;
    }
    Ok(channels)
}
