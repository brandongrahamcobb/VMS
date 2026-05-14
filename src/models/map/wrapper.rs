/* map/wrapper.rs
 * The purpose of this module is to wrap map models.
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
use crate::models::map::model::MapModel;
use crate::models::mob::model::Mob;
use crate::models::portal::wrapper::Portal;
use crate::wz::error::WzError;

#[derive(Clone)]
pub struct Map {
    pub model: MapModel,
    pub portals: Vec<Portal>,
    pub items: Vec<Item>,
    pub chars: Vec<Character>,
    pub mobs: Vec<Mob>,
}

impl Map {
    pub fn get_portal(&self, tn: String) -> Result<Portal, ModelError> {
        let portal: Portal = self
            .portals
            .iter()
            .find(|p| p.model.pn == tn)
            .cloned()
            .ok_or(WzError::ObjectError)?;
        Ok(portal)
    }
}
