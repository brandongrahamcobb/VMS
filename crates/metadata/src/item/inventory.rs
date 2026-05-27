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
use crate::item::{equip, nonequip};
use crate::service;
use entity::item::model::{InventoryTab, ItemModel, ItemWzInfo};
use inc::helpers;
use std::time::SystemTime;

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

pub fn create_item_model_by_wz(wz: i32) -> Result<ItemModel, ItemMetadataError> {
    let itab: InventoryTab = get_inventory_tab_by_wz(wz)?;
    let item_model = {
        match itab {
            InventoryTab::Equip => {
                let item_wz_info: ItemWzInfo = equip::build_equip_item_wz_info_by_wz(wz)?;
                ItemModel {
                    id: None,
                    char_id: None,
                    ipos: None,
                    strength: helpers::calculate_rand_stat(item_wz_info.strength, 5),
                    dexterity: helpers::calculate_rand_stat(item_wz_info.dexterity, 5),
                    intelligence: helpers::calculate_rand_stat(item_wz_info.intelligence, 5),
                    luck: helpers::calculate_rand_stat(item_wz_info.luck, 5),
                    attack: helpers::calculate_rand_stat(item_wz_info.attack, 5),
                    weapon_defense: helpers::calculate_rand_stat(item_wz_info.weapon_defense, 10),
                    magic: helpers::calculate_rand_stat(item_wz_info.magic, 5),
                    magic_defense: helpers::calculate_rand_stat(item_wz_info.magic_defense, 10),
                    hp: helpers::calculate_rand_stat(item_wz_info.hp, 10),
                    mp: helpers::calculate_rand_stat(item_wz_info.mp, 10),
                    accuracy: helpers::calculate_rand_stat(item_wz_info.accuracy, 5),
                    avoid: helpers::calculate_rand_stat(item_wz_info.avoid, 5),
                    hands: helpers::calculate_rand_stat(item_wz_info.hands, 5),
                    speed: helpers::calculate_rand_stat(item_wz_info.speed, 5),
                    jump: helpers::calculate_rand_stat(item_wz_info.jump, 5),
                    wz,
                    slots: 0,      //placeholder
                    expire: 0,     //placeholder
                    level: 0,      //placeholder
                    flag: 0,       //placeholder
                    item_level: 0, //placeholder
                    item_exp: 0,   //placeholder
                    vicious: 0,    //placeholder
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                }
            }
            _ => {
                let _: ItemWzInfo = nonequip::build_nonequip_item_wz_info_by_wz(wz)?; //placeholder
                ItemModel {
                    id: None,
                    char_id: None,
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
                }
            }
        }
    };
    Ok(item_model)
}
