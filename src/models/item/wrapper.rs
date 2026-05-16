/* item/wrapper.rs
 * The purpose of this module is to provide an item wrapper.
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

use crate::db::error::DatabaseError;
use crate::models::item;
use crate::models::item::cash_nonequip_model::CashNonEquipItemModel;
use crate::models::item::constants::InventoryTab;
use crate::models::item::equip_model::EquipItemModel;
use crate::models::item::error::ItemError;
use crate::models::item::etc_model::EtcItemModel;
use crate::models::item::model::ItemModel;
use crate::models::item::setup_model::SetupItemModel;
use crate::models::item::use_model::UseItemModel;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

pub struct Inventory {
    pub equipped_tab: HashMap<i16, Item>,
    pub equip_tab: HashMap<i16, Item>,
    pub use_tab: HashMap<i16, Item>,
    pub setup_tab: HashMap<i16, Item>,
    pub etc_tab: HashMap<i16, Item>,
    pub cash_tab: HashMap<i16, Item>,
}

pub enum EquipSlot {
    Android,
    Cash,
    Pet,
    Regular,
}

pub enum Item {
    CashEquip(EquipItem),
    CashNonEquip(CashNonEquipItem),
    Equip(EquipItem),
    Etc(EtcItem),
    Setup(SetupItem),
    Use(UseItem),
}

pub struct CashNonEquipItem {
    pub model: CashNonEquipItemModel,
}

pub struct EquipItem {
    pub model: EquipItemModel,
}

pub struct EtcItem {
    pub model: EtcItemModel,
}

pub struct SetupItem {
    pub model: SetupItemModel,
}

pub struct UseItem {
    pub model: UseItemModel,
}

impl Item {
    pub fn get_ipos(&self) -> Result<i16, ItemError> {
        match self {
            Item::Equip(i) => i.model.get_ipos(),
            Item::CashEquip(i) => i.model.get_ipos(),
            Item::CashNonEquip(i) => i.model.get_ipos(),
            Item::Use(i) => i.model.get_ipos(),
            Item::Setup(i) => i.model.get_ipos(),
            Item::Etc(i) => i.model.get_ipos(),
        }
    }
}

impl Inventory {
    pub async fn equip(&mut self, state: &SharedState, item: Item) -> Result<(), ItemError> {
        match item {
            Item::Equip(mut i) => {
                let old_pos = i.model.ipos.ok_or(ItemError::NoPos)?;
                self.equip_tab.remove(&old_pos);
                i.model.ipos = Some(item::service::get_equip_ipos_by_wz(i.model.wz)?);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                let new_pos = i.model.ipos.unwrap();
                self.equipped_tab.insert(new_pos, Item::Equip(i));
                Ok(())
            }
            Item::CashEquip(mut i) => {
                let old_pos = i.model.ipos.ok_or(ItemError::NoPos)?;
                self.equip_tab.remove(&old_pos);
                i.model.ipos = Some(item::service::get_equip_ipos_by_wz(i.model.wz)?);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                let new_pos = i.model.ipos.unwrap();
                self.equipped_tab.insert(new_pos, Item::CashEquip(i));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn unequip(&mut self, state: &SharedState, item: Item) -> Result<(), ItemError> {
        match item {
            Item::Equip(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = Some(self.next_free_pos(&inventory_tab)?);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                Ok(())
            }
            Item::CashEquip(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = Some(self.next_free_pos(&inventory_tab)?);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn pick_up(&mut self, state: &SharedState, item: Item) -> Result<i16, ItemError> {
        match item {
            Item::Equip(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                let pos = self.next_free_pos(&inventory_tab)?;
                i.model.ipos = Some(pos);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                self.equip_tab.insert(pos, Item::Equip(i));
                Ok(pos)
            }
            Item::Etc(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                let pos = self.next_free_pos(&inventory_tab)?;
                i.model.ipos = Some(pos);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                if let Some(pos) = i.model.ipos {
                    self.etc_tab.insert(pos, Item::Etc(i));
                }
                Ok(pos)
            }
            Item::CashEquip(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                let pos = self.next_free_pos(&inventory_tab)?;
                i.model.ipos = Some(pos);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                if let Some(pos) = i.model.ipos {
                    self.equip_tab.insert(pos, Item::CashEquip(i));
                }
                Ok(pos)
            }
            Item::Setup(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                let pos = self.next_free_pos(&inventory_tab)?;
                i.model.ipos = Some(pos);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                if let Some(pos) = i.model.ipos {
                    self.setup_tab.insert(pos, Item::Setup(i));
                }
                Ok(pos)
            }
            Item::Use(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                let pos = self.next_free_pos(&inventory_tab)?;
                i.model.ipos = Some(pos);
                i.model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                if let Some(pos) = i.model.ipos {
                    self.use_tab.insert(pos, Item::Use(i));
                }
                Ok(pos)
            }
            _ => Err(ItemError::TabError),
        }
    }

    pub fn next_free_pos(&self, tab: &InventoryTab) -> Result<i16, ItemError> {
        let tab = self.get_tab(tab);
        if let Some(pos) = (1..=96).find(|pos| !tab.contains_key(pos)) {
            return Ok(pos);
        } else {
            Err(ItemError::InventoryFull)
        }
    }

    pub fn get_tab(&self, tab: &InventoryTab) -> &HashMap<i16, Item> {
        match tab {
            InventoryTab::Equip => &self.equip_tab,
            InventoryTab::Use => &self.use_tab,
            InventoryTab::Setup => &self.setup_tab,
            InventoryTab::Etc => &self.etc_tab,
            InventoryTab::Cash => &self.cash_tab,
        }
    }
}
