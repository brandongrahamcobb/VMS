use std::time::SystemTime;

use crate::config::settings;
use crate::models::channel::error::ChannelError;
use crate::models::channel::model::{Channel, ChannelModel, NewChannelInsert};
use crate::models::error::ModelError;
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
    let flag: i16 = settings::get_world_flag()?;
    let mut id = 0;
    let count = channel_count;
    for count in 0..count {
        let port = world_port + 1 + count;
        let channel_model = ChannelModel {
            capacity: Some(capacity),
            id,
            flag: Some(flag),
            port,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        channels.push(Channel {
            model: channel_model,
        });
        id += 1;
    }
    Ok(channels)
}

impl Channel {
    pub fn new(model: ChannelModel) -> Self {
        Self { model }
    }
}

impl NewChannelInsert {
    pub fn default(id: i16, port: i16) -> Self {
        Self { id, port }
    }
}

impl ChannelModel {
    pub fn get_capacity(&self) -> Result<i16, ModelError> {
        if let Some(capacity) = self.capacity {
            Ok(capacity)
        } else {
            Err(ModelError::from(ChannelError::NoCapacity(self.id)))
        }
    }
}
