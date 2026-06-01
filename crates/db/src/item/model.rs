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
use crate::item::error::ItemModelError;
use crate::schema::{drops, items};
use base::inventory::InventoryTab;
use base::item::BaseItem;
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

impl ItemModel {
    pub fn default(char_id: i32, wz: i32) -> Result<ItemModel, ItemModelError> {
        let itab: InventoryTab = metadata::item::inventory::get_inventory_tab_by_wz(wz)?;
        let item_model: ItemModel = {
            match itab {
                InventoryTab::Equip => {
                    let base: BaseItem = metadata::item::equip::build_equip_item_wz_info_by_wz(wz)?;
                    ItemModel {
                        id: None,
                        char_id: Some(char_id),
                        ipos: None,
                        strength: base.strength,
                        dexterity: base.dexterity,
                        intelligence: base.intelligence,
                        luck: base.luck,
                        attack: base.attack,
                        weapon_defense: base.weapon_defense,
                        magic: base.magic,
                        magic_defense: base.magic_defense,
                        hp: base.hp,
                        mp: base.mp,
                        accuracy: base.accuracy,
                        avoid: base.avoid,
                        hands: base.hands,
                        speed: base.speed,
                        jump: base.jump,
                        wz,
                        slots: base.slots,
                        expire: 0,
                        level: 0,
                        item_level: 0,
                        flag: base.flag,
                        item_exp: 0,
                        vicious: 0,
                        created_at: Some(SystemTime::now()),
                        updated_at: SystemTime::now(),
                    }
                }
                _ => ItemModel {
                    id: None,
                    char_id: Some(char_id),
                    ipos: None,
                    strength: 0,
                    dexterity: 0,
                    intelligence: 0,
                    luck: 0,
                    attack: 0,
                    weapon_defense: 0,
                    magic: 0,
                    magic_defense: 0,
                    hp: 0,
                    mp: 0,
                    accuracy: 0,
                    avoid: 0,
                    hands: 0,
                    speed: 0,
                    jump: 0,
                    wz,
                    slots: 0,
                    expire: 0,
                    level: 0,
                    item_level: 0,
                    flag: 0,
                    item_exp: 0,
                    vicious: 0,
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                },
            }
        };
        Ok(item_model)
    }

    pub fn get_id(&self) -> Result<i32, ItemModelError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(ItemModelError::NoId)
        }
    }

    pub fn get_char_id(&self) -> Result<i32, ItemModelError> {
        if let Some(id) = self.char_id {
            Ok(id)
        } else {
            Err(ItemModelError::NoCharId(self.get_id()?))
        }
    }

    pub fn get_ipos(&self) -> Result<i16, ItemModelError> {
        if let Some(ipos) = self.ipos {
            Ok(ipos)
        } else {
            Err(ItemModelError::NoPos(self.get_id()?))
        }
    }

    pub fn get_created_at(&self) -> Result<SystemTime, ItemModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(ItemModelError::NoCreatedAt(self.get_id()?))
        }
    }
}
