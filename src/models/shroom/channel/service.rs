use crate::config::settings;
use crate::models::error::ModelError;
use crate::models::shroom::channel::error::ChannelError;
use crate::models::shroom::channel::model::{Channel, ChannelModel};
use crate::runtime::state::SharedState;

pub async fn get_channel_by_id(
    state: &SharedState,
    channel_id: i16,
) -> Result<Channel, ModelError> {
    let worlds = {
        let state = state.lock().await;
        state.worlds.clone()
    };
    for world in worlds {
        for channel in &world.channels {
            if channel.model.id == channel_id {
                return Ok(channel.clone());
            }
        }
        return Err(ModelError::from(ChannelError::NotFound(channel_id)));
    }
    Err(ModelError::from(ChannelError::UnexpectedError))
}

pub fn load_channels(channel_count: i16, world_port: i16) -> Result<Vec<Channel>, ModelError> {
    let mut channels: Vec<Channel> = Vec::new();
    let capacity: i16 = settings::get_channel_capacity()?;
    let flag: i16 = settings::get_channel_flag()?;
    let mut id = 0;
    let count = channel_count;
    for count in 0..count {
        let port = world_port + 1 + count;
        let channel_model = ChannelModel {
            capacity,
            id,
            flag,
            port,
        };
        channels.push(Channel {
            model: channel_model,
        });
        id += 1;
    }
    Ok(channels)
}
