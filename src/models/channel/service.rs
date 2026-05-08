use crate::config::settings;
use crate::models::channel::error::ChannelError;
use crate::models::channel::model::{Channel, ChannelModel};
use crate::models::error::ModelError;
use crate::models::world;
use crate::runtime::state::SharedState;

pub async fn get_channel_by_id(state: &SharedState, channel_id: i8) -> Result<Channel, ModelError> {
    let worlds = {
        let state = state.lock().await;
        state.worlds
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

pub fn load_channels(channel_count: i16, world_port: u16) -> Result<Vec<Channel>, ModelError> {
    let mut channels: Vec<Channel> = Vec::new();
    let capacity: i16 = settings::get_channel_capacity()?;
    let flag: i8 = settings::get_world_flag()?;
    let mut id = 0;
    let count = channel_count as u16;
    for count in 0..count {
        let port = world_port + 1 + count;
        let channel_model = ChannelModel {
            capacity: capacity,
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
