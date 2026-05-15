/* world/service.rs
 * The purpose of this module is to provide assisting functions and implementations for worlds.
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

use std::collections::HashMap;

use crate::config::settings;
use crate::constants::WORLDS;
use crate::models::channel;
use crate::models::error::ModelError;
use crate::models::world::error::WorldError;
use crate::models::world::wrapper::World;

pub fn load_worlds() -> Result<HashMap<i16, World>, ModelError> {
    let count = settings::get_world_count()? as usize;
    WORLDS
        .get(..count)
        .ok_or(ModelError::from(WorldError::CountExceedsAvailable))?
        .iter()
        .enumerate()
        .map(|(id, model)| {
            let channels = channel::service::load_channels(
                settings::get_channel_count()?,
                id as i16,
                model.base_port,
            )?;
            Ok((id as i16, World { model, channels }))
        })
        .collect()
}
