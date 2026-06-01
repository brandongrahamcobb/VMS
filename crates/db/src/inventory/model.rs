/* db/src/item/model.rs
 * The purpose of this module is to provide an item model and associated methods.
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
use crate::inventory::error::InventoryCapacityModelError;
use crate::schema::inventory_capacity;
use diesel::Queryable;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = inventory_capacity)]
pub struct InventoryCapacityModel {
    pub id: Option<i32>,
    pub char_id: i32,
    pub equip_slot_capacity: i16,
    pub use_slot_capacity: i16,
    pub etc_slot_capacity: i16,
    pub setup_slot_capacity: i16,
    pub cash_slot_capacity: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl InventoryCapacityModel {
    pub fn default(char_id: i32) -> InventoryCapacityModel {
        let capacity: i16 = 96;
        InventoryCapacityModel {
            id: None,
            char_id,
            equip_slot_capacity: capacity,
            use_slot_capacity: capacity,
            etc_slot_capacity: capacity,
            setup_slot_capacity: capacity,
            cash_slot_capacity: capacity,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        }
    }

    pub fn get_id(&self) -> Result<i32, InventoryCapacityModelError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(InventoryCapacityModelError::NoId)
        }
    }

    pub fn get_created_at(&self) -> Result<SystemTime, InventoryCapacityModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(InventoryCapacityModelError::NoCreatedAt(self.get_id()?))
        }
    }
}
