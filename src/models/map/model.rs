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
use crate::models::error::ModelError;
use crate::models::map::wrapper::Map;
// use crate::models::mob::model::{Mob, MobModel};
use crate::models::portal::model::PortalModel;
use crate::models::portal::wrapper::Portal;
use crate::runtime::state::SharedState;
// use crate::models::{mob, portal};
use crate::models::portal;

#[derive(Clone)]
pub struct MapModel {
    pub wz: i32,
}

impl MapModel {
    pub async fn load(&self, _state: &SharedState) -> Result<Map, ModelError> {
        let p_models: Vec<PortalModel> = portal::service::get_portal_models_by_map_wz(self.wz)?;
        let mut portals: Vec<Portal> = Vec::<Portal>::new();
        for p_model in p_models {
            portals.push(p_model.load()?);
        }
        // let mob_models: Vec<MobModel> = mob::service::get_mob_models_by_map_wz(self.wz)?;
        // let mobs: Vec<Mob> = Vec::<Mob>::new();
        // for mob_model in mob_models {
        //     mobs.push(mob_model.load()?);
        // }
        Ok(Map {
            model: self.clone(),
            portals: portals.clone(),
            // mobs: Vec::new(), //placeholder
        })
    }
}
