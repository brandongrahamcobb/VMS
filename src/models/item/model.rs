/* item/model.rs
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

use crate::db::schema::items;
use crate::models::error::ModelError;
use crate::models::item::error::ItemError;
use crate::models::item::wrapper::Item;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = items)]
pub struct ItemModel {
    pub id: Option<i32>,
    pub char_id: Option<i32>,
    pub equipped: bool,
    pub wz: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub luck: i32,
    pub attack: i32,
    pub weapon_defense: i32,
    pub magic: i32,
    pub magic_defense: i32,
    pub hp: i32,
    pub mp: i32,
    pub accuracy: i32,
    pub avoid: i32,
    pub hands: i32,
    pub speed: i32,
    pub jump: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl ItemModel {
    pub fn load(&self) -> Result<Item, ModelError> {
        Ok(Item {
            model: self.clone(),
        })
    }
    pub fn get_id(&self) -> Result<i32, ModelError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(ModelError::from(ItemError::NoId))
        }
    }
}

pub enum EquipType {
    AndroidEquipType(AndroidEquipType),
    CashEquipType(CashEquipType),
    PetEquipType(PetEquipType),
    RegularEquipType(RegularEquipType),
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
