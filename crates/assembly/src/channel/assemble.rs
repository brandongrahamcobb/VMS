/* assembly/src/channel/assemble.rs
 * The purpose of this module is to assemble a channel wrapper.
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

use crate::channel::error::ChannelAssemblyError;
use config::settings;
use entity::channel::model::ChannelModel;
use entity::channel::wrapper::Channel;
use std::collections::HashMap;

pub fn assemble_channels(
    count: i8,
    world_id: i16,
    base_port: i16,
) -> Result<HashMap<u8, Channel>, ChannelAssemblyError> {
    (0..count)
        .map(|id| {
            let port = base_port + (world_id * count as i16) + id as i16;
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
