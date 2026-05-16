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

use crate::db::error::DatabaseError;
use crate::metadata;
use crate::metadata::error::MetadataError;
use crate::models::item;
use crate::models::item::cash_nonequip_model::CashNonEquipItemModel;
use crate::models::item::constants::{CASH_EQUIP_SLOTS, InventoryTab, OTHER_EQUIP_SLOTS};
use crate::models::item::equip_model::EquipItemModel;
use crate::models::item::error::ItemError;
use crate::models::item::etc_model::EtcItemModel;
use crate::models::item::setup_model::SetupItemModel;
use crate::models::item::use_model::UseItemModel;
use crate::models::item::wrapper::Inventory;
use crate::models::item::wrapper::Item;
use crate::runtime::state::SharedState;
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
    let wz_cat = wz / 10000;
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
                ipos: None,
                wz,
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
                ipos: None,
                wz,
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
                ipos: None,
                wz,
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
                ipos: None,
                wz,
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
            let filename: String = String::from("Character.wz");
            let json = metadata::service::wz_to_img(wz, &filename)?;
            let item_model = EquipItemModel {
                id: None,
                char_id: None,
                wz,
                ipos: None,
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
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            };
            let item_model: EquipItemModel = item_model
                .update_item(state)
                .await
                .map_err(|e| DatabaseError::DieselError(e))?;
            if json["info"]["cash"] == 0 {
                Item::Equip(item_model.load())
            } else {
                Item::CashEquip(item_model.load())
            }
        }
    };
    Ok(item)
}

fn get_equip_stats_from_wz(root: &Value, key: &str) -> Option<i32> {
    let map = root.get("info").ok_or(MetadataError::ObjectError).ok()?;
    map.get(key).and_then(|v| v.as_i64().map(|n| n as i32))
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
            let equip_item = equip_item_model.load();
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
            let equip_item = equip_item_model.load();
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
