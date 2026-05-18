/* portal/service.rs
 * The purpose of this module is to provide assisting functions and implementations for portals.
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
use crate::models::portal::error::PortalError;
use crate::models::portal::model::PortalModel;
use std::collections::HashMap;

pub fn get_portal_models(map_wz: i32) -> Result<HashMap<u8, PortalModel>, PortalError> {
    let mut p_models: HashMap<u8, PortalModel> = HashMap::new();
    let portal_map = get_portal_json_map_from_map_wz(map_wz)?;
    for (pid, portal) in portal_map.iter() {
        let pid = pid.parse::<u8>().unwrap_or(0);
        let pn = portal["pn"].as_str().unwrap_or("").to_string();
        let tm = portal["tm"]
            .as_i64()
            .map(|v| v as i32)
            .ok_or(PortalError::NoTargetMap)?;
        let tn = portal["tn"].as_str().unwrap_or("sp").to_string();
        p_models.insert(pid, PortalModel { pid, pn, tm, tn });
    }
    Ok(p_models)
}

pub fn get_portal_json_map_from_map_wz(
    map_wz: i32,
) -> Result<serde_json::Map<String, serde_json::Value>, PortalError> {
    let filename: String = String::from("Map.wz");
    let json = metadata::service::wz_to_img(map_wz, &filename)?;
    let portal_map = json["portal"].as_object().ok_or(PortalError::NoPortal)?;
    Ok(portal_map.clone())
}
