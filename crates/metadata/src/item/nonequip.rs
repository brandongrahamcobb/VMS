/* metadata/src/item/nonequip.rs
 * The purpose of this module is to provide metadata access to nonequips.
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

use entity::item::model::{InventoryTab, ItemWzInfo};

use crate::item::error::ItemMetadataError;
use crate::item::inventory;
use crate::service;

pub fn build_nonequip_item_wz_info_by_wz(wz: i32) -> Result<ItemWzInfo, ItemMetadataError> {
    let itab: InventoryTab = inventory::get_inventory_tab_by_wz(wz)?;
    let filename: &str = "Item.wz";
    let modified_wz: i32 = wz.div_euclid(10000);
    let padded_wz: String = format!("{:08}", wz);
    let json = service::wz_to_img(modified_wz, filename)?;
    let item = json
        .get(padded_wz)
        .ok_or(ItemMetadataError::NonequipError)?;
    let cash = item
        .get("info")
        .ok_or(ItemMetadataError::NonequipError)?
        .get("cash")
        .map(|c| c == 1)
        .unwrap_or(false);
    let wz_info = ItemWzInfo {
        cash,
        islot: None,
        itab: itab as i8,
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
        slots: 0,
        flag: 0, //placeholder
    };
    Ok(wz_info)
}
