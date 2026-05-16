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

use crate::models::map::error::MapError;
use crate::models::map::model::MapModel;
use crate::models::map::wrapper::Map;
use crate::models::{mob, portal};
use std::collections::HashMap;

pub fn get_map_wz_by_job_id(job_id: i16) -> Result<i32, MapError> {
    match job_id {
        1 => Ok(10000),
        1000 => Ok(130000000),
        2000 => Ok(140000000),
        _ => Ok(0), //placeholder
    }
}

pub fn load_map(map_wz: i32) -> Result<Map, MapError> {
    let mobs = mob::service::load_mobs(map_wz)?;
    let portals = portal::service::load_portals(map_wz)?;
    Ok(Map {
        model: MapModel { wz: map_wz },
        chars: HashMap::new(),
        items: HashMap::new(),
        mobs,
        portals,
    })
}
