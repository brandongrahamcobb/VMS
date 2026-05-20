/* portal/model.rs
 * The purpose of this module is to provide a portal model.
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
use crate::models::portal::wrapper::Portal;

#[derive(Clone)]
pub struct PortalModel {
    pub map_wz: i32,
}

#[derive(Clone)]
pub struct PortalWzInfo {
    pub pid: u8,
    pub pn: String,
    pub tm: i32,
    pub tn: String,
}

impl PortalModel {
    pub fn load(&self, map_wz: i32, pid: u8) -> Result<Portal, PortalError> {
        let filename: String = String::from("Map.wz");
        let json = metadata::service::wz_to_img(map_wz, &filename)?;
        let portal_map = json["portal"].as_object().ok_or(PortalError::NoPortal)?;
        let portal = &portal_map[&pid.to_string()];
        let pn = portal["pn"].as_str().unwrap_or("").to_string();
        let tm = portal["tm"]
            .as_i64()
            .map(|v| v as i32)
            .ok_or(PortalError::NoTargetMap)?;
        let tn = portal["tn"].as_str().unwrap_or("sp").to_string();
        let wz_info: PortalWzInfo = PortalWzInfo { pid, pn, tm, tn };
        let portal = Portal {
            model: self.clone(),
            info: wz_info,
        };
        Ok(portal)
    }
}
