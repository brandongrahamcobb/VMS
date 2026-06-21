/* inc/src/item.rs
 * The purpose of this module is to provide helper methods for items.
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

use crate::error::IncError;
use base::{inventory::InventoryTab, item::BaseItem};
use config::settings;
use db;
use db::item::model::DropData;
use db::item::model::ItemModel;
use db::pool::DbPool;
use rand::RngExt;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn create_item_model_by_wz(wz: i32) -> Result<ItemModel, IncError> {
    let itab: InventoryTab = metadata::item::inventory::get_inventory_tab_by_wz(wz)?;
    let item_model = {
        match itab {
            InventoryTab::Equip => {
                let item_wz_info: BaseItem =
                    metadata::item::equip::build_equip_item_wz_info_by_wz(wz)?;
                ItemModel {
                    id: None,
                    char_id: None,
                    ipos: None,
                    itab: None,
                    strength: calculate_rand_stat(item_wz_info.strength, 5),
                    dexterity: calculate_rand_stat(item_wz_info.dexterity, 5),
                    intelligence: calculate_rand_stat(item_wz_info.intelligence, 5),
                    luck: calculate_rand_stat(item_wz_info.luck, 5),
                    attack: calculate_rand_stat(item_wz_info.attack, 5),
                    weapon_defense: calculate_rand_stat(item_wz_info.weapon_defense, 10),
                    magic: calculate_rand_stat(item_wz_info.magic, 5),
                    magic_defense: calculate_rand_stat(item_wz_info.magic_defense, 10),
                    hp: calculate_rand_stat(item_wz_info.hp, 10),
                    mp: calculate_rand_stat(item_wz_info.mp, 10),
                    accuracy: calculate_rand_stat(item_wz_info.accuracy, 5),
                    avoid: calculate_rand_stat(item_wz_info.avoid, 5),
                    hands: calculate_rand_stat(item_wz_info.hands, 5),
                    speed: calculate_rand_stat(item_wz_info.speed, 5),
                    jump: calculate_rand_stat(item_wz_info.jump, 5),
                    wz,
                    slots: 0,      //placeholder
                    expire: 0,     //placeholder
                    level: 0,      //placeholder
                    flag: 0,       //placeholder
                    item_level: 0, //placeholder
                    item_exp: 0,   //placeholder
                    vicious: 0,    //placeholder
                    equipped: false,
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                }
            }
            _ => ItemModel {
                id: None,
                char_id: None,
                ipos: None,
                itab: None,
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
                equipped: false,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            },
        }
    };
    Ok(item_model)
}

pub async fn get_random_drops(
    pool: &DbPool,
    mob_wz: i32,
) -> Result<HashMap<BaseItem, ItemModel>, IncError> {
    let drop_rate: f64 = settings::get_item_drop_rate()?;
    let drop_data: Vec<DropData> = db::item::getters::get_item_drop_data(pool, mob_wz).await?;
    let mut items: Vec<ItemModel> = Vec::new();
    let mut item_map: HashMap<BaseItem, ItemModel> = HashMap::new();
    for drop_entry in drop_data {
        let chance: f64 = (drop_entry.chance as f64 / 1_000_000.0) * drop_rate;
        let to_drop = if chance > 1.0 {
            true
        } else {
            rand::rng().random_bool(chance)
        };
        if to_drop {
            if chance > 2.0 {
                let multiplier = chance.div_euclid(1.0) as i32;
                for _ in 0..multiplier {
                    let drop_model: ItemModel =
                        crate::item::create_item_model_by_wz(drop_entry.item_wz)?;
                    let drop_models: Vec<ItemModel> =
                        db::item::setters::update_items(pool, vec![drop_model]).await?;
                    items.push(drop_models[0].clone());
                }
            } else {
                let drop_model: ItemModel =
                    crate::item::create_item_model_by_wz(drop_entry.item_wz)?;
                let drop_models: Vec<ItemModel> =
                    db::item::setters::update_items(pool, vec![drop_model]).await?;
                items.push(drop_models[0].clone());
            }
        }
    }
    for item_model in items {
        let itab = metadata::item::inventory::get_inventory_tab_by_wz(item_model.wz)?;
        let base_item: BaseItem = match itab {
            InventoryTab::Equip => {
                metadata::item::equip::build_equip_item_wz_info_by_wz(item_model.wz)?
            }
            _ => metadata::item::nonequip::build_nonequip_item_wz_info_by_wz(item_model.wz)?,
        };
        item_map.insert(base_item, item_model);
    }
    Ok(item_map)
}

pub fn build_base_item_by_char_id_and_item_wz(
    char_id: i32,
    wz: i32,
) -> Result<ItemModel, IncError> {
    let itab: InventoryTab = metadata::item::inventory::get_inventory_tab_by_wz(wz)?;
    let item_model: ItemModel = {
        match itab {
            InventoryTab::Equip => {
                let base: BaseItem = metadata::item::equip::build_equip_item_wz_info_by_wz(wz)?;
                ItemModel {
                    id: None,
                    char_id: Some(char_id),
                    itab: None,
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
                    slots: 7, // TODO: change slots by wz
                    expire: 0,
                    level: 0,
                    item_level: 0,
                    flag: base.flag,
                    item_exp: 0,
                    vicious: 0,
                    equipped: false,
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                }
            }
            _ => ItemModel {
                id: None,
                char_id: Some(char_id),
                itab: None,
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
                equipped: false,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            },
        }
    };
    Ok(item_model)
}

// pub struct InventoryMod {
//     pub mode: InventoryModMode,
//     pub inv_type: i8,
//     pub pos: i16,
//     pub count: i16,
//     pub char_name: String,
//     pub item_id: i32,
// }

// pub async fn equip(pool: &DbPool, inv: &mut Inventory, item_id: i32) -> Result<(), DomainError> {
//     let item_pos = inv
//         .equip_tab
//         .iter()
//         .find_map(|(pos, stack)| {
//             stack.iter().find_map(|item| {
//                 item.model
//                     .get_id()
//                     .ok()
//                     .filter(|id| *id == item_id)
//                     .map(|_| *pos)
//             })
//         })
//         .ok_or(DomainError::ItemError)?;
//     let mut stack = inv
//         .equip_tab
//         .remove(&item_pos)
//         .ok_or(DomainError::ItemError)?;
//     let item = stack
//         .iter_mut()
//         .find(|item| item.model.get_id().is_ok_and(|id| id == item_id))
//         .ok_or(DomainError::ItemError)?;
//
//     let new_pos = metadata::item::equip::get_equip_ipos_by_wz(item.model.wz)?;
//     item.model.ipos = Some(new_pos);
//     db::item::setters::update_item(pool, &item.model).await?;
//     inv.equipped_tab.insert(new_pos, stack);
//     Ok(())
// }
//
// pub async fn unequip(pool: &DbPool, inv: &mut Inventory, item_id: i32) -> Result<(), DomainError> {
//     let item = inv
//         .equipped_tab
//         .values()
//         .flatten()
//         .find(|item| item.model.get_id().is_ok_and(|id| id == item_id))
//         .ok_or(DomainError::ItemError)?;
//     let itab: InventoryTab = metadata::item::inventory::get_inventory_tab_by_wz(item.model.wz)?;
//     let new_pos = entity::item::service::next_free_pos(inv, &itab)?;
//     let item = inv
//         .equipped_tab
//         .values_mut()
//         .flatten()
//         .find(|item| item.model.get_id().is_ok_and(|id| id == item_id))
//         .ok_or(DomainError::ItemError)?;
//     item.model.ipos = Some(new_pos);
//     db::item::setters::update_item(pool, &item.model).await?;
//     Ok(())
// }
//
// pub async fn pick_up(
//     pool: &DbPool,
//     inv: &mut Inventory,
//     ign: String,
//     char_id: i32,
//     mut item: Item,
// ) -> Result<InventoryMod, DomainError> {
//     let item_model: ItemModel = item.model.clone();
//     let item_info: ItemWzInfo = item.info.clone();
//     let values = [&inv.cash_tab, &inv.etc_tab, &inv.setup_tab, &inv.use_tab];
//     let (pos, stack_size, itab) = values
//         .iter()
//         .find_map(|value| {
//             value.iter().find_map(|(pos, inv_item_stack)| {
//                 if inv_item_stack[0].model.wz == item.model.wz {
//                     Some((
//                         *pos,
//                         inv_item_stack.len(),
//                         metadata::item::inventory::get_inventory_tab_by_wz(item.model.wz).ok()?,
//                     ))
//                 } else {
//                     None
//                 }
//             })
//         })
//         .unwrap_or((
//             entity::item::service::next_free_pos(
//                 inv,
//                 &metadata::item::inventory::get_inventory_tab_by_wz(item.model.wz)?,
//             )?,
//             0,
//             metadata::item::inventory::get_inventory_tab_by_wz(item.model.wz)?,
//         ));
//
//     item.model.ipos = Some(pos);
//     item.model.char_id = Some(char_id);
//     db::item::setters::update_item(pool, &item.model).await?;
//     match itab {
//         InventoryTab::Equip => {
//             inv.equip_tab.insert(pos, vec![item]);
//         }
//         InventoryTab::Cash => {
//             inv.cash_tab.entry(pos).or_default().push(item);
//         }
//         InventoryTab::Use => {
//             inv.use_tab.entry(pos).or_default().push(item);
//         }
//         InventoryTab::Etc => {
//             inv.etc_tab.entry(pos).or_default().push(item);
//         }
//         InventoryTab::Setup => {
//             inv.setup_tab.entry(pos).or_default().push(item);
//         }
//     }
//     let inv_mod: InventoryMod = InventoryMod {
//         mode: InventoryModMode::Add,
//         inv_type: itab as i8,
//         pos,
//         count: stack_size as i16 + 1,
//         item_model: Some(item_model),
//         item_info: Some(item_info),
//         char_name: ign,
//     };
//     Ok(inv_mod)
// }

pub fn calculate_rand_stat(default_value: i16, max_range: i32) -> i16 {
    if default_value == 0 {
        return 0;
    }
    let l_max_range = ((default_value as f64 * 0.1).ceil() as i32).min(max_range);
    let rand = rand::rng().random::<f64>();
    ((default_value as i32 - l_max_range) + (rand * (l_max_range * 2 + 1) as f64).floor() as i32)
        as i16
}

pub fn calculate_rand_meso_amount(meso_rate: f64, level: i16) -> i32 {
    let base = (level * 3 + 20) as f32;
    let min = (base * 0.75) as i32;
    let max = (base * 1.25) as i32;
    let amount = rand::rng().random_range(min..=max);
    (amount as f64 * meso_rate) as i32
}
