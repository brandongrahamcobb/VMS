/* metadata/src/map/return.rs
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
use crate::service;

pub fn get_death_map_by_wz(map_wz: i32) -> Result<i32, MapMetadataError> {
    let filename: &str = "Map.wz";
    let json = service::wz_to_img(map_wz, &filename)?;
    let return_map_wz = json["info"]["returnMap"]
        .as_i64()
        .ok_or(MapMetadataError::DeathMapError)? as i32;
    Ok(return_map_wz)
}
