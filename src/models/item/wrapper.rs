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
use crate::models::item::constants::InventoryTab;
use crate::models::item::error::ItemError;
use crate::models::item::model::{InventoryMod, InventoryModMode, ItemModel, ItemWzInfo};
use crate::runtime::state::SharedState;
use std::collections::HashMap;

pub struct Inventory {
    pub equipped_tab: HashMap<i16, Vec<Item>>,
    pub equip_tab: HashMap<i16, Vec<Item>>,
    pub use_tab: HashMap<i16, Vec<Item>>,
    pub setup_tab: HashMap<i16, Vec<Item>>,
    pub etc_tab: HashMap<i16, Vec<Item>>,
    pub cash_tab: HashMap<i16, Vec<Item>>,
}

pub enum EquipSlot {
    Android,
    Cash,
    Pet,
    Regular,
}

pub struct Item {
    pub model: ItemModel,
    pub info: ItemWzInfo,
}

impl Item {
    pub fn get_ipos(&self) -> Result<i16, ItemError> {
        self.model.get_ipos()
    }
}

impl Inventory {
    pub async fn equip(&mut self, state: &SharedState, mut item: Item) -> Result<(), ItemError> {
        match item.model.ipos {
            Some(old_pos) => {
                self.equip_tab.remove(&old_pos).ok_or(ItemError::NoPos)?;
            }
            None => (),
        }
        item.model.ipos = Some(item::service::get_equip_ipos_by_wz(item.model.wz)?);
        item.model
            .update_item(state)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        let new_pos = item.model.ipos.unwrap();
        self.equipped_tab.insert(new_pos, vec![item]);
        Ok(())
    }

    pub async fn unequip(&mut self, state: &SharedState, mut item: Item) -> Result<(), ItemError> {
        let inventory_tab: InventoryTab = item::service::get_inventory_tab_by_wz(item.model.wz)?;
        item.model.ipos = Some(self.next_free_pos(&inventory_tab)?);
        item.model
            .update_item(state)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        Ok(())
    }

    pub async fn pick_up(
        &mut self,
        state: &SharedState,
        ign: String,
        char_id: i32,
        mut item: Item,
    ) -> Result<InventoryMod, ItemError> {
        let item_model: ItemModel = item.model.clone();
        let item_info: ItemWzInfo = item.info.clone();
        let values = [
            &self.cash_tab,
            &self.etc_tab,
            &self.setup_tab,
            &self.use_tab,
        ];
        let (pos, stack_size, itab) = values
            .iter()
            .find_map(|value| {
                value.iter().find_map(|(pos, inv_item_stack)| {
                    if inv_item_stack[0].model.wz == item.model.wz {
                        Some((
                            *pos,
                            inv_item_stack.len(),
                            item::service::get_inventory_tab_by_wz(item.model.wz).ok()?,
                        ))
                    } else {
                        None
                    }
                })
            })
            .unwrap_or((
                self.next_free_pos(&item::service::get_inventory_tab_by_wz(item.model.wz)?)?,
                0,
                item::service::get_inventory_tab_by_wz(item.model.wz)?,
            ));
        item.model.ipos = Some(pos);
        item.model.char_id = Some(char_id);
        item.model
            .update_item(state)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        match itab {
            InventoryTab::Equip => {
                self.equip_tab.insert(pos, vec![item]);
            }
            InventoryTab::Cash => {
                self.cash_tab.entry(pos).or_default().push(item);
            }
            InventoryTab::Use => {
                self.use_tab.entry(pos).or_default().push(item);
            }
            InventoryTab::Etc => {
                self.etc_tab.entry(pos).or_default().push(item);
            }
            InventoryTab::Setup => {
                self.setup_tab.entry(pos).or_default().push(item);
            }
        }

        let inv_mod: InventoryMod = InventoryMod {
            mode: InventoryModMode::Add,
            inv_type: itab as i8,
            pos,
            count: stack_size as i16 + 1,
            item_model: Some(item_model),
            item_info: Some(item_info),
            char_name: ign,
        };
        Ok(inv_mod)
    }

    pub fn next_free_pos(&self, tab: &InventoryTab) -> Result<i16, ItemError> {
        let tab = self.get_tab(tab);
        if let Some(pos) = (1..=96).find(|pos| !tab.contains_key(pos)) {
            return Ok(pos);
        } else {
            Err(ItemError::InventoryFull)
        }
    }

    pub fn get_tab(&self, tab: &InventoryTab) -> &HashMap<i16, Vec<Item>> {
        match tab {
            InventoryTab::Equip => &self.equip_tab,
            InventoryTab::Use => &self.use_tab,
            InventoryTab::Setup => &self.setup_tab,
            InventoryTab::Etc => &self.etc_tab,
            InventoryTab::Cash => &self.cash_tab,
        }
    }
}
