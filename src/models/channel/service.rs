use crate::models::channel::error::ChannelError;
use crate::models::channel::model::Channel;
use crate::models::world::model::World;

pub fn resolve_channel(
    channel_id: i16,
    world_id: i16,
    worlds: Vec<World>,
) -> Result<Channel, ChannelError> {
    for world in worlds {
        if world.id == world_id {
            for channel in &world.channels {
                if channel.channel_id == channel_id {
                    return Ok(channel.clone());
                }
            }
            return Err(ChannelError::NotFound(channel_id));
        }
    }
    Err(ChannelError::UnexpectedError)
}
