use crate::net::channel::error::ChannelError;
use crate::net::error::NetworkError;
use crate::net::world;

#[derive(Clone, Debug)]
pub struct Channel {
    pub world_id: i16,
    pub channel_id: i16,
    pub name: String,
    pub capacity: i16,
    pub port: i16,
}

pub fn resolve_channel(channel_id: i16, world_id: i16) -> Result<Channel, NetworkError> {
    let worlds = world::core::load_worlds()?;
    for world in worlds {
        if world.id == world_id {
            for channel in &world.channels {
                if channel.channel_id == channel_id {
                    return Ok(channel.clone());
                }
            }
            return Err(NetworkError::from(ChannelError::NotFound(channel_id)));
        }
    }
    Err(NetworkError::UnexpectedError)
}
