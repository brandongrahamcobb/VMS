/* assembly/src/map/assemble.rs
 * The purpose of this module is to assemble a map wrapper.
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
use entity::map::model::{MapModel, MapWzInfo, VacancyState};
use entity::map::wrapper::Map;
use entity::mob::wrapper::Mob;
use entity::portal::wrapper::Portal;
use metadata;
use std::collections::HashMap;

use crate::map::error::MapAssemblyError;
use crate::mob;
use crate::portal;

pub fn assemble_map_by_map_wz(map_wz: i32) -> Result<Map, MapAssemblyError> {
    let map_model: MapModel = MapModel { wz: map_wz };
    let map_wz_info: MapWzInfo = assemble_map_wz_info(map_wz)?;
    let mobs: HashMap<u32, Mob> = mob::assemble::assemble_mobs_by_map_wz(map_wz)?;
    let portals: HashMap<u8, Portal> = portal::assemble::assemble_portals_by_map_wz(map_wz)?;
    // let (tick_tx, _) = broadcast::channel(32);
    Ok(Map {
        chars: HashMap::new(),
        info: map_wz_info,
        items: HashMap::new(),
        mobs,
        model: map_model,
        portals,
        // tick_tx,
        vacancy_token: None,
        vacancy: VacancyState::Vacant,
    })
}

pub fn assemble_map_wz_info(map_wz: i32) -> Result<MapWzInfo, MapAssemblyError> {
    let death_map_wz = metadata::map::death::get_death_map_by_wz(map_wz)?;
    let mob_rate = metadata::map::mob::get_mob_rate_by_map_wz(map_wz)?;
    Ok(MapWzInfo {
        death_map_wz,
        mob_rate,
        wz: map_wz,
    })
}
