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
use crate::models::item::wrapper::Item;
use crate::models::map::error::MapError;
use crate::models::map::model::MapModel;
use crate::models::mob::wrapper::Mob;
use crate::models::portal::wrapper::Portal;
use std::collections::HashMap;

pub struct Map {
    pub model: MapModel,
    pub chars: HashMap<i32, Character>,
    pub items: HashMap<i32, Item>,
    pub mobs: HashMap<u32, Mob>,
    pub portals: HashMap<u8, Portal>,
    pub next_mob_id: u32,
    pub free_mob_ids: Vec<u32>,
}

impl Map {
    pub fn get_portal(&self, tn: String) -> Result<&Portal, MapError> {
        let portal: &Portal = self
            .portals
            .values()
            .find(|p| p.model.pn == tn)
            .ok_or(MetadataError::ObjectError)?;
        Ok(portal)
    }

    pub fn next_mob_id(&mut self) -> u32 {
        if let Some(id) = self.free_mob_ids.pop() {
            id
        } else {
            let id = self.next_mob_id;
            self.next_mob_id += 1;
            id
        }
    }

    pub fn spawn_mob(&mut self, mob: Mob) -> u32 {
        let id = self.next_mob_id();
        self.mobs.insert(id, mob);
        id
    }

    pub fn kill_mob(&mut self, id: u32) {
        if self.mobs.remove(&id).is_some() {
            self.free_mob_ids.push(id);
        }
    }
}
