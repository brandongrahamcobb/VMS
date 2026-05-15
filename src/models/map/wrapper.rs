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

use crate::metadata::error::MetadataError;
use crate::models::character::wrapper::Character;
use crate::models::error::ModelError;
use crate::models::item::wrapper::Item;
use crate::models::map::model::MapModel;
use crate::models::mob::wrapper::Mob;
use crate::models::portal::wrapper::Portal;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Map {
    pub model: MapModel,
    pub chars: HashMap<i32, Character>,
    pub items: HashMap<i32, Item>,
    pub mobs: HashMap<u32, Mob>,
    pub portals: HashMap<u8, Portal>,
}

impl Map {
    pub fn get_portal(&self, tn: String) -> Result<Portal, ModelError> {
        let portal: Portal = self
            .portals
            .values()
            .find(|p| p.model.pn == tn)
            .cloned()
            .ok_or(MetadataError::ObjectError)?;
        Ok(portal)
    }
}
