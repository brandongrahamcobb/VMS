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
use crate::models::error::ModelError;
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

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Item {
    CashEquip(EquipItem),
    CashNonEquip(CashNonEquipItem),
    Equip(EquipItem),
    Etc(EtcItem),
    Setup(SetupItem),
    Use(UseItem),
}

#[derive(Clone)]
pub struct CashNonEquipItem {
    pub model: CashNonEquipItemModel,
}

#[derive(Clone)]
pub struct EquipItem {
    pub model: EquipItemModel,
}

#[derive(Clone)]
pub struct EtcItem {
    pub model: EtcItemModel,
}

#[derive(Clone)]
pub struct SetupItem {
    pub model: SetupItemModel,
}

#[derive(Clone)]
pub struct UseItem {
    pub model: UseItemModel,
}

impl Item {
    pub fn get_ipos(&self) -> Result<i16, ModelError> {
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
    pub async fn equip(&mut self, state: &SharedState, item: Item) -> Result<(), ModelError> {
        match item.clone() {
            Item::Equip(mut i) => {
                self.equip_tab.remove(&item.get_ipos()?);
                i.model.ipos = Some(item::service::get_equip_ipos_by_wz(i.model.wz)?);
                i.model.update_item(state).await?;
                self.equipped_tab.insert(i.model.ipos.unwrap(), item);
                Ok(())
            }
            Item::CashEquip(mut i) => {
                self.equip_tab.remove(&item.get_ipos()?);
                i.model.ipos = Some(item::service::get_equip_ipos_by_wz(i.model.wz)?);
                i.model.update_item(state).await?;
                self.equipped_tab.insert(i.model.ipos.unwrap(), item);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn unequip(&mut self, state: &SharedState, item: Item) -> Result<(), ModelError> {
        match item {
            Item::Equip(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = self.next_free_pos(&inventory_tab);
                i.model.update_item(state).await?;
                Ok(())
            }
            Item::CashEquip(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = self.next_free_pos(&inventory_tab);
                i.model.update_item(state).await?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn pick_up(&mut self, state: &SharedState, item: Item) -> Result<Item, ModelError> {
        match item {
            Item::Equip(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = self.next_free_pos(&inventory_tab);
                i.model.update_item(state).await?;
                if let Some(pos) = i.model.ipos {
                    self.equip_tab.insert(pos, Item::Equip(i.clone()));
                }
                Ok(Item::Equip(i.clone()))
            }
            Item::Etc(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = self.next_free_pos(&inventory_tab);
                i.model.update_item(state).await?;
                if let Some(pos) = i.model.ipos {
                    self.etc_tab.insert(pos, Item::Etc(i.clone()));
                }
                Ok(Item::Etc(i.clone()))
            }
            Item::CashEquip(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = self.next_free_pos(&inventory_tab);
                i.model.update_item(state).await?;
                if let Some(pos) = i.model.ipos {
                    self.equip_tab.insert(pos, Item::CashEquip(i.clone()));
                }
                Ok(Item::CashEquip(i.clone()))
            }
            Item::Setup(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = self.next_free_pos(&inventory_tab);
                i.model.update_item(state).await?;
                if let Some(pos) = i.model.ipos {
                    self.setup_tab.insert(pos, Item::Setup(i.clone()));
                }
                Ok(Item::Setup(i.clone()))
            }
            Item::Use(mut i) => {
                let inventory_tab: InventoryTab =
                    item::service::get_inventory_tab_by_wz(i.model.wz)?;
                i.model.ipos = self.next_free_pos(&inventory_tab);
                i.model.update_item(state).await?;
                if let Some(pos) = i.model.ipos {
                    self.use_tab.insert(pos, Item::Use(i.clone()));
                }
                Ok(Item::Use(i.clone()))
            }
            _ => Err(ModelError::from(ItemError::TabError)),
        }
    }

    pub fn next_free_pos(&self, tab: &InventoryTab) -> Option<i16> {
        let tab = match tab {
            InventoryTab::Equip => &self.equip_tab,
            InventoryTab::Use => &self.use_tab,
            InventoryTab::Setup => &self.setup_tab,
            InventoryTab::Etc => &self.etc_tab,
            InventoryTab::Cash => &self.cash_tab,
        };
        (1..=96).find(|pos| !tab.contains_key(pos))
    }
}
