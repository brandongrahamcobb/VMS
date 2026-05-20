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
use crate::models::item::cash_nonequip_model::CashNonEquipItemModel;
use crate::models::item::constants::{CASH_EQUIP_SLOTS, InventoryTab, OTHER_EQUIP_SLOTS};
use crate::models::item::equip_model::{EquipItemModel, EquipItemWz};
use crate::models::item::error::ItemError;
use crate::models::item::etc_model::EtcItemModel;
use crate::models::item::model::DropData;
use crate::models::item::setup_model::SetupItemModel;
use crate::models::item::use_model::UseItemModel;
use crate::models::item::wrapper::Item;
use crate::models::item::wrapper::{EquipItem, Inventory};
use crate::models::mob::model::{MobWzInfo, MobWzLife};
use crate::runtime::state::SharedState;
use rand::RngExt;
use serde_json::Value;
use std::collections::HashMap;
use std::time::SystemTime;

pub async fn load_inventory(state: &SharedState, char_id: i32) -> Result<Inventory, ItemError> {
    let mut equipped_tab: HashMap<i16, Item> = HashMap::new();
    let equipped_items = item::service::get_equipped_items_by_char_id(state, char_id).await?;
    for equipped_item in equipped_items {
        equipped_tab.insert(equipped_item.get_ipos()?, equipped_item);
    }
    let mut equip_tab: HashMap<i16, Item> = HashMap::new();
    let equip_items = item::service::get_equip_items_by_char_id(state, char_id).await?;
    for equip_item in equip_items {
        equip_tab.insert(equip_item.get_ipos()?, equip_item);
    }
    let mut use_tab: HashMap<i16, Item> = HashMap::new();
    let use_items = item::service::get_use_items_by_char_id(state, char_id).await?;
    for use_item in use_items {
        use_tab.insert(use_item.get_ipos()?, use_item);
    }
    let mut etc_tab: HashMap<i16, Item> = HashMap::new();
    let etc_items = item::service::get_etc_items_by_char_id(state, char_id).await?;
    for etc_item in etc_items {
        etc_tab.insert(etc_item.get_ipos()?, etc_item);
    }
    let mut setup_tab: HashMap<i16, Item> = HashMap::new();
    let setup_items = item::service::get_setup_items_by_char_id(state, char_id).await?;
    for setup_item in setup_items {
        setup_tab.insert(setup_item.get_ipos()?, setup_item);
    }
    let mut cash_tab: HashMap<i16, Item> = HashMap::new();
    let cash_nonequip_items =
        item::service::get_cash_nonequip_items_by_char_id(state, char_id).await?;
    for cash_nonequip_item in cash_nonequip_items {
        cash_tab.insert(cash_nonequip_item.get_ipos()?, cash_nonequip_item);
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
        .ok_or(ItemError::InvalidISlot)?;
    let cash = json["info"]["cash"]
        .as_i64()
        .ok_or(ItemError::InvalidCash)?;
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
    let item = match itab {
        InventoryTab::Use => {
            let item_model = UseItemModel {
                id: None,
                char_id: None,
                wz,
                ipos: None,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            };
            let item_model: UseItemModel = item_model
                .update_item(state)
                .await
                .map_err(|e| DatabaseError::DieselError(e))?;
            Item::Use(item_model.load())
        }
        InventoryTab::Setup => {
            let item_model = SetupItemModel {
                id: None,
                char_id: None,
                wz,
                ipos: None,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            };
            let item_model: SetupItemModel = item_model
                .update_item(state)
                .await
                .map_err(|e| DatabaseError::DieselError(e))?;
            Item::Setup(item_model.load())
        }
        InventoryTab::Cash => {
            let item_model = CashNonEquipItemModel {
                id: None,
                char_id: None,
                wz,
                ipos: None,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            };
            let item_model: CashNonEquipItemModel = item_model
                .update_item(state)
                .await
                .map_err(|e| DatabaseError::DieselError(e))?;
            Item::CashNonEquip(item_model.load())
        }
        InventoryTab::Etc => {
            let item_model = EtcItemModel {
                id: None,
                char_id: None,
                wz,
                ipos: None,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            };
            let item_model: EtcItemModel = item_model
                .update_item(state)
                .await
                .map_err(|e| DatabaseError::DieselError(e))?;
            Item::Etc(item_model.load())
        }
        InventoryTab::Equip => {
            let wz_info = build_equip_item_wz_info(wz)?;
            let item_model: EquipItemModel = EquipItemModel {
                id: None,
                char_id: None,
                wz,
                ipos: None,
                strength: rand_stat(wz_info.strength, 5),
                dexterity: rand_stat(wz_info.dexterity, 5),
                intelligence: rand_stat(wz_info.intelligence, 5),
                luck: rand_stat(wz_info.luck, 5),
                attack: rand_stat(wz_info.attack, 5),
                weapon_defense: rand_stat(wz_info.weapon_defense, 10),
                magic: rand_stat(wz_info.magic, 5),
                magic_defense: rand_stat(wz_info.magic_defense, 10),
                hp: rand_stat(wz_info.hp, 10),
                mp: rand_stat(wz_info.mp, 10),
                accuracy: rand_stat(wz_info.accuracy, 5),
                avoid: rand_stat(wz_info.avoid, 5),
                hands: rand_stat(wz_info.hands, 5),
                speed: rand_stat(wz_info.speed, 5),
                jump: rand_stat(wz_info.jump, 5),
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            };
            let item_model: EquipItemModel = item_model
                .update_item(state)
                .await
                .map_err(|e| DatabaseError::DieselError(e))?;
            if !wz_info.cash {
                Item::Equip(item_model.load()?)
            } else {
                Item::CashEquip(item_model.load()?)
            }
        }
    };
    Ok(item)
}

fn get_equip_stats_from_wz(root: &Value, key: &str) -> Option<i16> {
    let map = root.get("info").ok_or(MetadataError::ObjectError).ok()?;
    map.get(key).and_then(|v| v.as_i64().map(|n| n as i16))
}

async fn get_equipped_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Item>, ItemError> {
    let mut equipped_items: Vec<Item> = Vec::<Item>::new();
    let equip_item_models = item::query::getters::get_equip_item_models_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    for equip_item_model in equip_item_models {
        if equip_item_model.ipos.unwrap() < 0 {
            let equip_item = equip_item_model.load()?;
            let filename: String = String::from("Character.wz");
            let json = metadata::service::wz_to_img(equip_item_model.wz, &filename)?;
            if json["info"]["cash"] == 0 {
                equipped_items.push(Item::Equip(equip_item));
            } else {
                equipped_items.push(Item::CashEquip(equip_item));
            }
        }
    }
    Ok(equipped_items)
}

async fn get_equip_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Item>, ItemError> {
    let mut equip_items: Vec<Item> = Vec::<Item>::new();
    let equip_item_models = item::query::getters::get_equip_item_models_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    for equip_item_model in equip_item_models {
        if equip_item_model.ipos.unwrap() > 0 {
            let equip_item = equip_item_model.load()?;
            let filename: String = String::from("Character.wz");
            let json = metadata::service::wz_to_img(equip_item_model.wz, &filename)?;
            if json["info"]["cash"] == 0 {
                equip_items.push(Item::Equip(equip_item));
            } else {
                equip_items.push(Item::CashEquip(equip_item));
            }
        }
    }
    Ok(equip_items)
}

async fn get_use_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Item>, ItemError> {
    let mut use_items: Vec<Item> = Vec::<Item>::new();
    let use_item_models = item::query::getters::get_use_item_models_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    for use_item_model in use_item_models {
        let use_item = use_item_model.load();
        use_items.push(Item::Use(use_item));
    }
    Ok(use_items)
}

async fn get_etc_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Item>, ItemError> {
    let mut etc_items: Vec<Item> = Vec::<Item>::new();
    let etc_item_models = item::query::getters::get_etc_item_models_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    for etc_item_model in etc_item_models {
        let etc_item = etc_item_model.load();
        etc_items.push(Item::Etc(etc_item));
    }
    Ok(etc_items)
}

async fn get_setup_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Item>, ItemError> {
    let mut setup_items: Vec<Item> = Vec::<Item>::new();
    let setup_item_models = item::query::getters::get_setup_item_models_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    for setup_item_model in setup_item_models {
        let setup_item = setup_item_model.load();
        setup_items.push(Item::Setup(setup_item));
    }
    Ok(setup_items)
}

async fn get_cash_nonequip_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Item>, ItemError> {
    let mut cash_nonequip_items: Vec<Item> = Vec::<Item>::new();
    let cash_nonequip_item_models =
        item::query::getters::get_cash_nonequip_item_models_by_char_id(state, char_id)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
    for cash_nonequip_item_model in cash_nonequip_item_models {
        let cash_nonequip_item = cash_nonequip_item_model.load();
        cash_nonequip_items.push(Item::CashNonEquip(cash_nonequip_item));
    }
    Ok(cash_nonequip_items)
}

pub fn build_equip_item_wz_info(item_wz: i32) -> Result<EquipItemWz, ItemError> {
    let filename: String = String::from("Character.wz");
    dbg!(item_wz);
    let json = metadata::service::wz_to_img(item_wz, &filename)?;
    let islot = json["info"]["islot"]
        .as_str()
        .ok_or(ItemError::InvalidISlot)?
        .to_string();
    let cash = json["info"]["cash"] == 1;
    let wz_info = EquipItemWz {
        cash,
        islot,
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
    if let Ok(item_model) =
        item::query::getters::get_etc_item_models_by_item_id(state, item_id).await
    {
        return Ok(Item::Etc(item_model.load()));
    }

    if let Ok(item_model) =
        item::query::getters::get_cash_nonequip_item_models_by_item_id(state, item_id).await
    {
        return Ok(Item::CashNonEquip(item_model.load()));
    }

    if let Ok(item_model) =
        item::query::getters::get_equip_item_models_by_item_id(state, item_id).await
    {
        let item: EquipItem = item_model.load()?;
        match get_inventory_tab_by_wz(item.model.wz)? {
            InventoryTab::Equip => return Ok(Item::Equip(item)),
            InventoryTab::Cash => return Ok(Item::CashEquip(item)),
            _ => (),
        }
    }

    if let Ok(item_model) =
        item::query::getters::get_setup_item_models_by_item_id(state, item_id).await
    {
        return Ok(Item::Setup(item_model.load()));
    }

    if let Ok(item_model) =
        item::query::getters::get_use_item_models_by_item_id(state, item_id).await
    {
        return Ok(Item::Use(item_model.load()));
    }
    Err(ItemError::ItemNotFound)
}
