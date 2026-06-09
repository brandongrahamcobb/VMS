/* metadata/src/map/map.rs
 * The purpose of this module is to provide metadata access to a map return map.
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

use crate::map::error::MapMetadataError;
use base::map::BaseMap;

pub fn get_map_wz_by_job_id(job_id: i16) -> i32 {
    match job_id {
        0 => 10000,
        1000 => 130000000,
        2000 => 140000000,
        _ => 0, //placeholder
    }
}

pub fn build_base_map_by_wz(wz: i32) -> Result<BaseMap, MapMetadataError> {
    let death_map_wz = crate::map::death::get_death_map_by_wz(wz)?;
    let mob_rate = crate::map::mob::get_mob_rate_by_map_wz(wz)?;
    Ok(BaseMap {
        death_map_wz,
        mob_rate,
        wz,
    })
}
