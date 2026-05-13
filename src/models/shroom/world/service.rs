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

use crate::config::settings;
use crate::constants::WORLDS;
use crate::models::error::ModelError;
use crate::models::shroom::channel;
use crate::models::shroom::world::error::WorldError;
use crate::models::shroom::world::wrapper::World;
use crate::runtime::state::SharedState;

pub fn load_worlds() -> Result<Vec<World>, ModelError> {
    let mut worlds: Vec<World> = Vec::new();
    let pairs: Vec<(i16, i16)> = settings::get_channel_world_pairs()?;
    for (id, count) in pairs {
        let world = if let Some(world_model) = WORLDS.get(id as usize) {
            world_model.clone()
        } else {
            return Err(ModelError::from(WorldError::NotFound(id)));
        };
        let channels = channel::service::load_channels(count, world.port)?;
        worlds.push(World {
            model: world,
            channels,
        })
    }
    Ok(worlds)
}

pub async fn get_world_by_id(state: &SharedState, world_id: i16) -> Result<World, ModelError> {
    let worlds = {
        let state = state.lock().await;
        state.worlds.clone()
    };
    for world in worlds {
        if world.model.id == world_id {
            return Ok(world.clone());
        }
        return Err(ModelError::from(WorldError::NotFound(world_id)));
    }
    Err(ModelError::from(WorldError::NoWorlds))
}
