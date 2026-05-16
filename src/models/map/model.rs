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
use crate::models::map::wrapper::Map;
use crate::models::mob::wrapper::Mob;
use crate::models::portal::wrapper::Portal;
use crate::models::{mob, portal};
use crate::runtime::state::SharedState;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MapModel {
    pub wz: i32,
}

impl MapModel {
    pub async fn load(&self, _state: &SharedState) -> Result<Map, MapError> {
        let portals: HashMap<u8, Portal> = portal::service::load_portals(self.wz)?;
        let (mobs, next_mob_id): (HashMap<u32, Mob>, u32) = mob::service::load_mobs(self.wz)?;
        Ok(Map {
            chars: HashMap::new(),
            items: HashMap::new(),
            model: self.clone(),
            mobs,
            portals,
            next_mob_id,
            free_mob_ids: Vec::new(),
        })
    }
}
