/* inventory/service.rs
 * The purpose of this module is to provide assisting functions for inventory items.
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
use crate::models::item::inventory::model::{EquipType, RegularEquipType};
use crate::models::item::inventory::wrapper::InventoryItem;
use crate::runtime::state::SharedState;
use crate::wz;

pub async fn get_inventory_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<InventoryItem>, ModelError> {
    let mut items: Vec<InventoryItem> = Vec::<InventoryItem>::new();
    let inventory_item_models =
        item::inventory::query::getters::get_inventory_item_models_by_char_id(state, char_id)
            .await?;
    for inventory_item_model in inventory_item_models {
        items.push(inventory_item_model.load()?)
    }
    Ok(items)
}

pub fn get_equip_type_from_wz(wz_id: i32) -> Result<EquipType, ModelError> {
    let category = wz::service::get_img_root(wz_id, "Item.wz");
    dbg!(category);
    return EquipType(RegularEquipType::Hat);
}
