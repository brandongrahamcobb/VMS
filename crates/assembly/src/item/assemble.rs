/* assembly/src/item/assemble.rs
 * The purpose of this module is to assemble an item wrapper and inventory wrapper.
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

use crate::item::error::ItemAssemblyError;
use db;
use db::pool::DbPool;
use entity::item::model::{InventoryTab, ItemModel, ItemWzInfo};
use entity::item::wrapper::{Inventory, Item};
use metadata;
use std::collections::HashMap;

pub async fn assemble_inventory_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<Inventory, ItemAssemblyError> {
    let item_models: Vec<ItemModel> =
        db::item::getters::get_item_models_by_char_id(pool, char_id).await?;
    let mut items: Vec<Item> = Vec::new();
    for item_model in item_models {
        items.push(assemble_item_by_id(pool, item_model.get_id()?).await?);
    }
    let mut cash_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut equipped_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut equip_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut etc_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut setup_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut use_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    for item in items {
        let ipos: i16 = item.model.get_ipos()?;
        let cash: i8 = InventoryTab::Cash as i8;
        let consume: i8 = InventoryTab::Use as i8;
        let equip: i8 = InventoryTab::Equip as i8;
        let etc: i8 = InventoryTab::Etc as i8;
        let setup: i8 = InventoryTab::Setup as i8;
        match item.info.itab {
            x if x == cash => {
                cash_tab.entry(ipos).or_default().push(item);
            }
            x if x == consume => {
                use_tab.entry(ipos).or_default().push(item);
            }
            x if x == equip => {
                if ipos < 0 {
                    equipped_tab.insert(ipos, vec![item]);
                } else {
                    equip_tab.insert(ipos, vec![item]);
                }
            }
            x if x == etc => {
                etc_tab.entry(ipos).or_default().push(item);
            }
            x if x == setup => {
                setup_tab.entry(ipos).or_default().push(item);
            }
            _ => return Err(ItemAssemblyError::InventoryError),
        }
    }
    let inventory: Inventory = Inventory {
        equipped_tab,
        equip_tab,
        use_tab,
        setup_tab,
        etc_tab,
        cash_tab,
    };
    Ok(inventory)
}

pub async fn assemble_item_by_id(pool: &DbPool, item_id: i32) -> Result<Item, ItemAssemblyError> {
    let item_model: ItemModel = db::item::getters::get_item_model_by_item_id(pool, item_id).await?;
    let itab: InventoryTab = metadata::item::inventory::get_inventory_tab_by_wz(item_model.wz)?;
    let item_wz_info: ItemWzInfo = {
        match itab {
            InventoryTab::Equip => {
                metadata::item::equip::build_equip_item_wz_info_by_wz(item_model.wz)?
            }
            _ => metadata::item::nonequip::build_nonequip_item_wz_info_by_wz(item_model.wz)?,
        }
    };
    let item: Item = Item {
        model: item_model,
        info: item_wz_info,
    };
    Ok(item)
}

pub async fn assemble_item_by_wz_info(
    pool: &DbPool,
    item_wz_info: ItemWzInfo,
) -> Result<Item, ItemAssemblyError> {
    let item_model: ItemModel = db::item::getters::get_item_model_by_item_id(pool, item_id).await?;
    let itab: InventoryTab = metadata::item::inventory::get_inventory_tab_by_wz(item_model.wz)?;
    let item_wz_info: ItemWzInfo = {
        match itab {
            InventoryTab::Equip => {
                metadata::item::equip::build_equip_item_wz_info_by_wz(item_model.wz)?
            }
            _ => metadata::item::nonequip::build_nonequip_item_wz_info_by_wz(item_model.wz)?,
        }
    };
    let item: Item = Item {
        model: item_model,
        info: item_wz_info,
    };
    Ok(item)
}
