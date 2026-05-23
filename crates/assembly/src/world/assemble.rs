/* assembly/src/world/assemble.rs
 * The purpose of this module is to assemble a world wrapper.
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

use crate::channel;
use crate::world::constants::WORLDS;
use crate::world::error::WorldAssemblyError;
use config::settings;
use entity::world::wrapper::World;
use std::collections::HashMap;

pub fn assemble_worlds() -> Result<HashMap<i16, World>, WorldAssemblyError> {
    let count = settings::get_world_count()? as usize;
    WORLDS
        .get(..count)
        .ok_or(WorldAssemblyError::CountExceedsAvailable)?
        .iter()
        .enumerate()
        .map(|(id, model)| {
            let channels = channel::assemble::assemble_channels(
                settings::get_channel_count()?,
                id as i16,
                model.base_port,
            )?;
            Ok((id as i16, World { model, channels }))
        })
        .collect()
}
