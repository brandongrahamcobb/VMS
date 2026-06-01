/* metadata/src/item/inventory.rs
 * The purpose of this module is to provide metadata access to inventory.
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

use crate::item::error::ItemMetadataError;
use crate::service;
use base::inventory::InventoryTab;

pub fn get_inventory_tab_by_wz(wz: i32) -> Result<InventoryTab, ItemMetadataError> {
    let filename: &str = "Item.wz";
    let wz_cat: i32 = wz.div_euclid(10000);
    let json = match service::wz_to_tree(wz_cat, filename) {
        Ok(json) => json,
        Err(_) => return Ok(InventoryTab::Equip),
    };
    if let Some(cat) = json[filename]
        .as_object()
        .and_then(|o| o.keys().next())
        .map(|k| k.as_str())
    {
        match cat {
            "Cash" | "Pet" | "Special" => return Ok(InventoryTab::Cash),
            "Consume" => return Ok(InventoryTab::Use),
            "Etc" => return Ok(InventoryTab::Etc),
            "Install" => return Ok(InventoryTab::Setup),
            _ => return Ok(InventoryTab::Equip),
        }
    }
    Err(ItemMetadataError::InventoryError)
}
