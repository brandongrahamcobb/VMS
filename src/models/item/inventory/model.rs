/* inventory/model.rs
 * The purpose of this module is to provide an inventory item model.
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

use crate::db::schema::inventory_items;
use crate::models::error::ModelError;
use crate::models::item::equip_stats;
use crate::models::item::equip_stats::model::EquipStatsModel;
use crate::models::item::inventory::wrapper::InventoryItem;
use crate::runtime::state::SharedState;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = inventory_items)]
pub struct InventoryItemModel {
    pub id: i32,
    pub equipped: bool,
    pub wz: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl InventoryItemModel {
    pub async fn load(&self, state: &SharedState) -> Result<InventoryItem, ModelError> {
        let equip_stats_model: EquipStatsModel =
            equip_stats::query::getters::get_equip_stats_model_by_id(state, self.id).await?;
        Ok(InventoryItem {
            model: self.clone(),
            equip_stats: equip_stats_model.load()?.clone(),
        })
    }
}

pub enum EquipType {
    AndroidEquipType,
    CashEquipType,
    PetEquipType,
    RegularEquipType,
}

pub enum AndroidEquipType {
    AndroidHat = 201,
    AndroidFace = 202,
    AndroidTop = 203,
    AndroidBottom = 204,
    AndroidGloves = 205,
    AndroidCape = 206,
}

pub enum RegularEquipType {
    Hat = 1,
    FaceAcc = 2,
    EyeAcc = 3,
    EarAcc = 4,
    Top = 5,
    Bottom = 6,
    Shoes = 7,
    Gloves = 8,
    Cape = 9,
    Shield = 10,
    Weapon = 11,
    RingOne = 12,
    RingTwo = 13,
    RingThree = 15,
    RingFour = 16,
    PendantOne = 17,
    TamedMob = 18,
    Saddle = 19,
    Medal = 49,
    Belt = 50,
    Pocket = 51,
    Book = 52,
    PendantTwo = 53,
    Shoulder = 54,
    Android = 55,
    Emblem = 56,
    Badge = 57,
    Subweapon = 58,
    Heart = 59,
}

pub enum CashEquipType {
    Hat = 101,
    FaceAcc = 102,
    EyeAcc = 103,
    EarAcc = 104,
    Top = 105,
    Bottom = 106,
    Shoes = 107,
    Gloves = 108,
    Cape = 109,
    Weapon = 111,
    RingOne = 112,
    RingTwo = 113,
    RingThree = 115,
    RingFour = 116,
}

pub enum PetEquipType {
    AccOne = 301,
    AccTwo = 302,
    AccThree = 303,
}
