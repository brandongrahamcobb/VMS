/* map/model.rs
 * The purpose of this module is to provide a map model and its methods.
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

use crate::models::map::error::MapError;
use crate::models::map::respawn;
use crate::models::map::wrapper::Map;
use crate::models::mob::model::MobModel;
use crate::models::mob::wrapper::Mob;
use crate::models::portal::model::PortalModel;
use crate::models::portal::wrapper::Portal;
use crate::models::{mob, portal};
use crate::runtime::state::SharedState;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MapModel {
    pub wz: i32,
}

impl MapModel {
    pub fn load_mobs(&self, map_wz: i32) -> Result<HashMap<u32, Mob>, MapError> {
        let m_models: HashMap<u32, MobModel> = mob::service::get_mob_models(map_wz)?;
        let mut mobs: HashMap<u32, Mob> = HashMap::new();
        for (mid, m_model) in m_models {
            mobs.insert(mid, m_model.load()?);
        }
        Ok(mobs)
    }

    pub fn load_portals(&self, map_wz: i32) -> Result<HashMap<u8, Portal>, MapError> {
        let p_models: HashMap<u8, PortalModel> = portal::service::get_portal_models(map_wz)?;
        let mut portals: HashMap<u8, Portal> = HashMap::new();
        for (pid, p_model) in p_models {
            portals.insert(pid, p_model.load()?);
        }
        Ok(portals)
    }

    pub async fn load(
        self,
        state: &SharedState,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
    ) -> Result<Map, MapError> {
        let mobs: HashMap<u32, Mob> = self.load_mobs(map_wz)?;
        let portals: HashMap<u8, Portal> = self.load_portals(map_wz)?;
        respawn::respawn_tick(state, world_id, channel_id, map_wz).await?;
        Ok(Map {
            model: MapModel { wz: map_wz },
            chars: HashMap::new(),
            dead_mobs: HashMap::new(),
            items: HashMap::new(),
            mobs,
            portals,
        })
    }
}
