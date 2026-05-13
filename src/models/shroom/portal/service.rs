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

use crate::models::error::ModelError;
use crate::models::shroom::portal::model::PortalModel;
use crate::models::shroom::portal::wrapper::Portal;
use crate::wz;
use crate::wz::error::WzError;

pub fn get_portal_models_by_map_wz(map_wz: i32) -> Result<Vec<PortalModel>, ModelError> {
    let root = wz::service::get_img_root(map_wz, "Map.wz")?;
    let wz_portals = root.get("portal").and_then(|p| p.as_object()).unwrap();
    let mut portal_models: Vec<PortalModel> = Vec::<PortalModel>::new();
    for (key, target) in wz_portals {
        let pid = key.parse::<i16>().unwrap_or(0);
        let pn = target
            .get("pn")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let tm = target
            .get("tm")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or(WzError::ObjectError)?;
        let tn = target
            .get("tn")
            .and_then(|v| v.as_str())
            .unwrap_or("sp")
            .to_string();
        portal_models.push(PortalModel { pid, pn, tm, tn });
    }
    Ok(portal_models)
}

pub fn load_portals(map_wz: i32) -> Result<Vec<Portal>, ModelError> {
    let portals: Vec<Portal> = Vec::<Portal>::new();
    Ok(portals)
}
