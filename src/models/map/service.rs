/* map/service.rs
 * The purpose of this module is to provide assisting functions and implementations for maps.
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

use crate::models::error::ModelError;
use crate::models::map::error::MapError;
use crate::models::map::model::MapModel;
use crate::models::map::wrapper::Map;
// use crate::models::mob::model::Mob;
use crate::models::portal::wrapper::Portal;
// use crate::models::{mob, portal};
use crate::models::portal;
use crate::runtime::state::SharedState;
use crate::wz;

pub async fn get_map_by_world_channel_map_wzs(
    state: &SharedState,
    world_id: i16,
    channel_id: i16,
    map_wz: i32,
) -> Result<Map, ModelError> {
    let worlds = {
        let state = state.lock().await;
        state.worlds.clone()
    };
    for world in worlds {
        if world.model.id == world_id {
            let channels = world.channels;
            for channel in channels {
                if channel.model.id == channel_id {
                    let maps = channel.maps;
                    for map in maps {
                        if map.model.wz == map_wz {
                            return Ok(map);
                        }
                    }
                }
            }
        }
    }
    Err(ModelError::from(MapError::NotFound(
        world_id, channel_id, map_wz,
    )))
}

pub fn get_map_wz_by_job_id(job_id: i16) -> Result<i32, ModelError> {
    match job_id {
        1 => Ok(10000),
        1000 => Ok(130000000),
        2000 => Ok(140000000),
        _ => Ok(0), //placeholder
    }
}

pub fn load_maps() -> Result<Vec<Map>, ModelError> {
    let root = wz::service::get_img_root(5, "Mob.wz")?;
    let map_wzs: Vec<i32> = root
        .get("mob")
        .and_then(|s| s.as_object())
        .unwrap_or(&serde_json::Map::new())
        .keys()
        .filter_map(|k| k.parse::<i32>().ok())
        .collect();
    dbg!(map_wzs.clone());
    let mut maps: Vec<Map> = Vec::<Map>::new();
    for map_wz in map_wzs {
        let portals: Vec<Portal> = portal::service::load_portals(map_wz)?;
        // let mobs: Vec<Mob> = mob::service::load_mobs(map_wz)?;
        maps.push(Map {
            portals: portals.clone(),
            // mobs: Vec::new(), //mobs.clone(), placeholder
            model: MapModel { wz: map_wz },
        });
    }
    Ok(maps)
}
