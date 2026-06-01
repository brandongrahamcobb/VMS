/* metadata/src/map/portal.rs
 * The purpose of this module is to provide metadata access to map portals.
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
use base::map::Point;
use base::portal::BasePortal;

pub fn get_portal_wz_info_by_map_wz_and_pid(
    map_wz: i32,
    pid: u8,
) -> Result<BasePortal, MapMetadataError> {
    let filename: String = String::from("Map.wz");
    let json = service::wz_to_img(map_wz, &filename)?;
    let portal_map = json["portal"]
        .as_object()
        .ok_or(MapMetadataError::PortalError)?;
    let portal = &portal_map[&pid.to_string()];
    let pn = portal["pn"].as_str().unwrap_or("").to_string();
    let tm = portal["tm"]
        .as_i64()
        .map(|v| v as i32)
        .ok_or(MapMetadataError::PortalError)?;
    let tn = portal["tn"].as_str().unwrap_or("sp").to_string();
    Ok(BasePortal {
        portal_wz: pid,
        portal_name: pn,
        target_map_wz: tm,
        target_portal_name: tn,
    })
}

pub fn get_zeroeth_portal_spawnpoint(map_wz: i32) -> Result<Point, MapMetadataError> {
    let filename: String = String::from("Map.wz");
    let json = service::wz_to_img(map_wz, &filename)?;
    let portal_map = json["portal"]
        .as_object()
        .ok_or(MapMetadataError::PortalError)?;
    let x = portal_map["0"]["x"].as_i64().unwrap_or(0) as i16;
    let y = portal_map["0"]["y"].as_i64().unwrap_or(0) as i16;
    Ok(Point { x, y })
}

pub fn get_portal_ids_by_map_wz(map_wz: i32) -> Result<Vec<u8>, MapMetadataError> {
    let mut portal_ids: Vec<u8> = Vec::new();
    let filename: String = String::from("Map.wz");
    let json = service::wz_to_img(map_wz, &filename)?;
    let portal_map = json["portal"]
        .as_object()
        .ok_or(MapMetadataError::PortalError)?;
    for (pid, _) in portal_map {
        let pid: u8 = pid.parse::<u8>()?;
        portal_ids.push(pid);
    }
    Ok(portal_ids)
}
