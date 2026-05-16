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
use crate::models::channel::error::ChannelError;
use crate::models::channel::model::ChannelModel;
use crate::models::channel::wrapper::Channel;
use std::collections::HashMap;

pub fn load_channels(
    count: i8,
    world_id: i16,
    base_port: i16,
) -> Result<HashMap<u8, Channel>, ChannelError> {
    (0..count)
        .map(|id| {
            let port = base_port + (world_id as i16 * count as i16) + id as i16;
            let flag = settings::get_channel_flag()?;
            let capacity = settings::get_channel_capacity()?;
            Ok((
                id as u8,
                Channel {
                    model: ChannelModel {
                        capacity,
                        world_id,
                        port,
                        flag,
                    },
                    maps: HashMap::new(),
                },
            ))
        })
        .collect()
}
