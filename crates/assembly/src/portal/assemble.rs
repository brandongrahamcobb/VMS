/* entity/src/portal/assemble.rs
 * The purpose of this module is to assemble a portal.
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

use crate::portal::error::PortalAssemblyError;
use entity::portal::model::{PortalModel, PortalWzInfo};
use entity::portal::wrapper::Portal;
use metadata;

pub fn assemble_portal_by_map_wz_and_pid(
    map_wz: i32,
    pid: u8,
) -> Result<Portal, PortalAssemblyError> {
    let portal_model: PortalModel = PortalModel { map_wz };
    let wz_info: PortalWzInfo =
        metadata::map::portal::get_portal_wz_info_by_map_wz_and_pid(map_wz, pid)?;
    let portal = Portal {
        model: portal_model,
        info: wz_info,
    };
    Ok(portal)
}

pub fn assemble_portals_by_map_wz(map_wz: i32) -> Result<HashMap<u8, Portal>, PortalAssemblyError> {
    let mut portals: HashMap<u8, Portal> = HashMap::new();
    let pids: Vec<u8> = metadata::map::portal::get_portal_ids_by_map_wz(map_wz)?;
    for pid in pids {
        let p: Portal = assemble_portal_by_map_wz_and_pid(map_wz, pid)?;
        portals.insert(pid, p);
    }
    Ok(portals)
}
