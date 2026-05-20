/* item/equip_model.rs
 * The purpose of this module is to provide an equip item model and its implementation.
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

use crate::db::schema::equip_items;
use crate::models::item;
use crate::models::item::error::ItemError;
use crate::models::item::model::ItemModel;
use crate::models::item::wrapper::EquipItem;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = equip_items)]
pub struct EquipItemModel {
    pub id: Option<i32>,
    pub char_id: Option<i32>,
    pub wz: i32,
    pub ipos: Option<i16>,
    pub strength: i16,
    pub dexterity: i16,
    pub intelligence: i16,
    pub luck: i16,
    pub attack: i16,
    pub weapon_defense: i16,
    pub magic: i16,
    pub magic_defense: i16,
    pub hp: i16,
    pub mp: i16,
    pub accuracy: i16,
    pub avoid: i16,
    pub hands: i16,
    pub speed: i16,
    pub jump: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

pub struct EquipItemWz {
    pub strength: i16,
    pub dexterity: i16,
    pub intelligence: i16,
    pub luck: i16,
    pub attack: i16,
    pub weapon_defense: i16,
    pub magic: i16,
    pub magic_defense: i16,
    pub hp: i16,
    pub mp: i16,
    pub accuracy: i16,
    pub avoid: i16,
    pub hands: i16,
    pub speed: i16,
    pub jump: i16,
    pub islot: String,
    pub cash: bool,
}

impl EquipItemModel {
    pub fn load(&self) -> Result<EquipItem, ItemError> {
        let wz_info: EquipItemWz = item::service::build_equip_item_wz_info(self.wz)?;
        Ok(EquipItem {
            model: self.clone(),
            info: wz_info,
        })
    }
}

impl ItemModel for EquipItemModel {
    fn id(&self) -> Option<i32> {
        self.id
    }
    fn ipos(&self) -> Option<i16> {
        self.ipos
    }
}
