/* channel/service.rs
 * The purpose of this module is to provide assisting functions and implementations for channels.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::config::settings;
use crate::models::error::ModelError;
use crate::models::shroom::channel::error::ChannelError;
use crate::models::shroom::channel::model::ChannelModel;
use crate::models::shroom::channel::wrapper::Channel;
use crate::models::shroom::map;
use crate::models::shroom::map::wrapper::Map;
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
    for i in 0..channel_count {
        let port = world_port + 1 + i;
        let channel_model = ChannelModel {
            capacity,
            id,
            flag,
            port,
        };
        let maps: Vec<Map> = map::service::load_maps()?;
        channels.push(Channel {
            model: channel_model,
            maps: maps.clone(),
        });
        id += 1;
    }
    Ok(channels)
}
