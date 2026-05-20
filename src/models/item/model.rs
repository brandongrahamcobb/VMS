/* item/model.rs
 * The purpose of this module is to provide an item model.
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
use crate::db::schema::{drops, items};
use crate::models::item::error::ItemError;
use crate::models::item::wrapper::Item;
use diesel::prelude::*;
use diesel::{Identifiable, Queryable};
use std::time::SystemTime;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = drops)]
pub struct DropData {
    pub id: i64,
    pub mob_wz: i32,
    pub item_wz: i32,
    pub minimum_quantity: i32,
    pub maximum_quantity: i32,
    pub chance: i32,
}

#[derive(Clone, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = items)]
pub struct ItemModel {
    pub id: Option<i32>,
    pub char_id: Option<i32>,
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
    pub wz: i32,
    pub slots: i32,
    pub expire: i64,
    pub level: i16,
    pub item_level: i16,
    pub flag: i16,
    pub item_exp: i16,
    pub vicious: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct ItemWzInfo {
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
    pub islot: Option<String>,
    pub cash: bool,
    pub itab: i8,
    pub slots: i32,
    pub flag: i16,
}

#[derive(Clone)]
pub enum InventoryModMode {
    Add = 0,
    ChangeCount = 1,
    Swap = 2,
    Remove = 3,
}

pub struct InventoryMod {
    pub mode: InventoryModMode,
    pub inv_type: i8,
    pub pos: i16,
    pub count: i16,
    pub char_name: String,
    pub item_model: Option<ItemModel>,
    pub item_info: Option<ItemWzInfo>,
}

impl InventoryMod {
    pub fn get_item_model(&self) -> Result<ItemModel, ItemError> {
        if let Some(model) = self.item_model.clone() {
            Ok(model)
        } else {
            Err(ItemError::NoItemModel)
        }
    }

    pub fn get_item_info(&self) -> Result<ItemWzInfo, ItemError> {
        if let Some(info) = self.item_info.clone() {
            Ok(info)
        } else {
            Err(ItemError::NoItemInfo)
        }
    }
}

impl ItemModel {
    pub fn get_id(&self) -> Result<i32, ItemError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(ItemError::NoId)
        }
    }

    pub fn get_ipos(&self) -> Result<i16, ItemError> {
        if let Some(ipos) = self.ipos {
            Ok(ipos)
        } else {
            Err(ItemError::NoPos)
        }
    }

    pub fn load(&self, item_wz_info: ItemWzInfo) -> Result<Item, ItemError> {
        Ok(Item {
            model: self.clone(),
            info: item_wz_info,
        })
    }
}
