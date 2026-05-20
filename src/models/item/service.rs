/* item/service.rs
 * The purpose of this module is to provide assisting functions for items.
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

use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::metadata;
use crate::metadata::error::MetadataError;
use crate::models::item;
use crate::models::item::constants::{CASH_EQUIP_SLOTS, InventoryTab, OTHER_EQUIP_SLOTS};
use crate::models::item::error::ItemError;
use crate::models::item::model::{DropData, ItemModel, ItemWzInfo};
use crate::models::item::wrapper::Inventory;
use crate::models::item::wrapper::Item;
use crate::models::mob::model::{MobWzInfo, MobWzLife};
use crate::runtime::state::SharedState;
use rand::RngExt;
use serde_json::Value;
use std::collections::HashMap;
use std::time::SystemTime;

pub async fn load_inventory(state: &SharedState, char_id: i32) -> Result<Inventory, ItemError> {
    let items = item::service::get_items_by_char_id(state, char_id).await?;
    let mut cash_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut equipped_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut equip_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut etc_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut setup_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    let mut use_tab: HashMap<i16, Vec<Item>> = HashMap::new();
    dbg!("test");
    for item in items {
        dbg!("test");
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
                    dbg!("test2");
                    equipped_tab.insert(ipos, vec![item]);
                } else {
                    dbg!("test3");
                    equip_tab.insert(ipos, vec![item]);
                }
            }
            x if x == etc => {
                etc_tab.entry(ipos).or_default().push(item);
            }
            x if x == setup => {
                setup_tab.entry(ipos).or_default().push(item);
            }
            _ => return Err(ItemError::TabError),
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

pub fn get_equip_ipos_by_wz(wz: i32) -> Result<i16, ItemError> {
    let filename: String = String::from("Character.wz");
    let json = metadata::service::wz_to_img(wz, &filename)?;
    let islot = json["info"]["islot"]
        .as_str()
        .ok_or(MetadataError::EntryError)?;
    let cash = json["info"]["cash"]
        .as_i64()
        .ok_or(MetadataError::EntryError)?;
    if cash == 0 {
        return OTHER_EQUIP_SLOTS
            .iter()
            .flat_map(|group| group.iter())
            .find(|slot| slot.islot == islot)
            .map(|slot| slot.key)
            .ok_or(ItemError::InvalidISlot);
    } else {
        return CASH_EQUIP_SLOTS
            .iter()
            .find(|slot| slot.islot == islot)
            .map(|slot| slot.key)
            .ok_or(ItemError::InvalidISlot);
    }
}

pub fn get_inventory_tab_by_wz(wz: i32) -> Result<InventoryTab, ItemError> {
    let filename: &str = "Item.wz";
    let wz_cat: i32 = wz.div_euclid(10000);
    let json = match metadata::service::wz_to_tree(wz_cat, filename) {
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
    return Err(ItemError::InvalidISlot);
}

pub async fn create_item(state: &SharedState, wz: i32) -> Result<Item, ItemError> {
    let itab: InventoryTab = get_inventory_tab_by_wz(wz)?;
    let item = {
        match itab {
            InventoryTab::Equip => {
                let item_wz_info: ItemWzInfo = build_equip_item_wz_info(wz)?;
                let item_model: ItemModel = ItemModel {
                    id: None,
                    char_id: None,
                    ipos: None,
                    strength: rand_stat(item_wz_info.strength, 5),
                    dexterity: rand_stat(item_wz_info.dexterity, 5),
                    intelligence: rand_stat(item_wz_info.intelligence, 5),
                    luck: rand_stat(item_wz_info.luck, 5),
                    attack: rand_stat(item_wz_info.attack, 5),
                    weapon_defense: rand_stat(item_wz_info.weapon_defense, 10),
                    magic: rand_stat(item_wz_info.magic, 5),
                    magic_defense: rand_stat(item_wz_info.magic_defense, 10),
                    hp: rand_stat(item_wz_info.hp, 10),
                    mp: rand_stat(item_wz_info.mp, 10),
                    accuracy: rand_stat(item_wz_info.accuracy, 5),
                    avoid: rand_stat(item_wz_info.avoid, 5),
                    hands: rand_stat(item_wz_info.hands, 5),
                    speed: rand_stat(item_wz_info.speed, 5),
                    jump: rand_stat(item_wz_info.jump, 5),
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
                };
                let item_model: ItemModel = item_model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                item_model.load(item_wz_info)?
            }
            _ => {
                let item_wz_info: ItemWzInfo = build_nonequip_item_wz_info(wz)?;
                let item_model: ItemModel = ItemModel {
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
                };
                let item_model: ItemModel = item_model
                    .update_item(state)
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
                item_model.load(item_wz_info)?
            }
        }
    };
    Ok(item)
}

fn get_equip_stats_from_wz(root: &Value, key: &str) -> Option<i16> {
    let map = root.get("info").ok_or(MetadataError::ObjectError).ok()?;
    map.get(key).and_then(|v| v.as_i64().map(|n| n as i16))
}

async fn get_items_by_char_id(state: &SharedState, char_id: i32) -> Result<Vec<Item>, ItemError> {
    let mut items: Vec<Item> = Vec::<Item>::new();
    let item_models = item::query::getters::get_item_models_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    for item_model in item_models {
        let itab: InventoryTab = get_inventory_tab_by_wz(item_model.wz)?;
        let item_wz_info: ItemWzInfo = {
            match itab {
                InventoryTab::Equip => build_equip_item_wz_info(item_model.wz)?,
                _ => build_nonequip_item_wz_info(item_model.wz)?,
            }
        };
        items.push(item_model.load(item_wz_info)?);
    }
    Ok(items)
}

pub fn build_equip_item_wz_info(wz: i32) -> Result<ItemWzInfo, ItemError> {
    let itab: InventoryTab = get_inventory_tab_by_wz(wz)?;
    let filename: String = String::from("Character.wz");
    let json = metadata::service::wz_to_img(wz, &filename)?;
    let islot = json["info"]["islot"]
        .as_str()
        .ok_or(ItemError::InvalidISlot)?
        .to_string();
    let cash = json["info"]["cash"] == 1;
    let wz_info = ItemWzInfo {
        cash,
        islot: Some(islot),
        itab: itab as i8,
        strength: get_equip_stats_from_wz(&json, "incSTR").unwrap_or(0),
        dexterity: get_equip_stats_from_wz(&json, "incDEX").unwrap_or(0),
        intelligence: get_equip_stats_from_wz(&json, "incINT").unwrap_or(0),
        luck: get_equip_stats_from_wz(&json, "incLUK").unwrap_or(0),
        attack: get_equip_stats_from_wz(&json, "incPAD").unwrap_or(0),
        weapon_defense: get_equip_stats_from_wz(&json, "incPDD").unwrap_or(0),
        magic: get_equip_stats_from_wz(&json, "incMAD").unwrap_or(0),
        magic_defense: get_equip_stats_from_wz(&json, "incMDD").unwrap_or(0),
        hp: get_equip_stats_from_wz(&json, "incMHP").unwrap_or(0),
        mp: get_equip_stats_from_wz(&json, "incMMP").unwrap_or(0),
        accuracy: get_equip_stats_from_wz(&json, "incACC").unwrap_or(0),
        avoid: get_equip_stats_from_wz(&json, "incEVA").unwrap_or(0),
        hands: get_equip_stats_from_wz(&json, "incHANDS").unwrap_or(0),
        speed: get_equip_stats_from_wz(&json, "incSPEED").unwrap_or(0),
        jump: get_equip_stats_from_wz(&json, "incJUMP").unwrap_or(0),
        slots: 0, // placeholder
        flag: 0,  //placeholder
    };
    Ok(wz_info)
}

pub fn build_nonequip_item_wz_info(wz: i32) -> Result<ItemWzInfo, ItemError> {
    let itab: InventoryTab = get_inventory_tab_by_wz(wz)?;
    let filename: &str = "Item.wz";
    let modified_wz: i32 = wz.div_euclid(10000);
    let json = metadata::service::wz_to_img(modified_wz, filename)?;
    let cash = json["info"]["cash"] == 1;
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

pub fn rand_stat(default_value: i16, max_range: i32) -> i16 {
    if default_value == 0 {
        return 0;
    }
    let l_max_range = ((default_value as f64 * 0.1).ceil() as i32).min(max_range);
    let rand = rand::rng().random::<f64>();
    ((default_value as i32 - l_max_range) + (rand * (l_max_range * 2 + 1) as f64).floor() as i32)
        as i16
}

pub async fn get_random_drops(
    state: &SharedState,
    mob_wz_life: MobWzLife,
) -> Result<HashMap<i32, Item>, ItemError> {
    let drop_rate: f64 = settings::get_item_drop_rate()?;
    let drop_data: Vec<DropData> = item::query::getters::get_item_drop_data(state, mob_wz_life.wz)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    let mut items: HashMap<i32, Item> = HashMap::new();
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
                items.insert(multiplier, create_item(state, drop_entry.item_wz).await?);
            } else {
                items.insert(1, create_item(state, drop_entry.item_wz).await?);
            }
        }
    }
    Ok(items)
}

pub async fn get_random_meso_drop(mob_wz_info: MobWzInfo) -> Result<i32, ItemError> {
    let meso_rate: f64 = settings::get_meso_drop_rate()?;
    let base = (mob_wz_info.level * 3 + 20) as f32;
    let min = (base * 0.75) as i32;
    let max = (base * 1.25) as i32;
    let amount = rand::rng().random_range(min..=max);
    Ok((amount as f64 * meso_rate) as i32)
}

pub async fn get_item_by_item_id(state: &SharedState, item_id: i32) -> Result<Item, ItemError> {
    let item_model: ItemModel = item::query::getters::get_item_model_by_item_id(state, item_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    let itab: InventoryTab = get_inventory_tab_by_wz(item_model.wz)?;
    let item_wz_info = {
        match itab {
            InventoryTab::Equip => build_equip_item_wz_info(item_model.wz)?,
            _ => build_nonequip_item_wz_info(item_model.wz)?,
        }
    };
    return Ok(item_model.load(item_wz_info)?);
}
