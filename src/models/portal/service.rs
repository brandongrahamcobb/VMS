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

use std::collections::HashMap;

use crate::metadata;
use crate::models::error::ModelError;
use crate::models::portal::error::PortalError;
use crate::models::portal::model::PortalModel;
use crate::models::portal::wrapper::Portal;

pub fn load_portals(map_wz: i32) -> Result<HashMap<u8, Portal>, ModelError> {
    let filename: String = String::from("Map.wz");
    let json = metadata::service::wz_to_img(map_wz, &filename)?;
    let wz_portals = json.get("portal").and_then(|p| p.as_object()).unwrap();
    let mut portals: HashMap<u8, Portal> = HashMap::new();
    for (key, target) in wz_portals {
        let pid = key.parse::<u8>().unwrap_or(0);
        let pn = target
            .get("pn")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let tm = target
            .get("tm")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or(PortalError::NoTargetMap)?;
        let tn = target
            .get("tn")
            .and_then(|v| v.as_str())
            .unwrap_or("sp")
            .to_string();
        portals.insert(pid, (PortalModel { pid, pn, tm, tn }).load()?);
    }
    Ok(portals)
}
