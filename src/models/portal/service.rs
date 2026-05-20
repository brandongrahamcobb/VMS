/* portal/service.rs
 * The purpose of this module is to provide assisting functions for portals.
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

use crate::metadata;
use crate::models::map::model::Point;
use crate::models::portal::error::PortalError;

pub fn get_zeroeth_portal_spawnpoint(map_wz: i32) -> Result<Point, PortalError> {
    let filename: String = String::from("Map.wz");
    let json = metadata::service::wz_to_img(map_wz, &filename)?;
    let portal_map = json["portal"].as_object().ok_or(PortalError::NoPortal)?;
    let x = portal_map["0"]["x"].as_i64().unwrap_or(0) as i16;
    let y = portal_map["0"]["y"].as_i64().unwrap_or(0) as i16;
    Ok(Point { x, y })
}
