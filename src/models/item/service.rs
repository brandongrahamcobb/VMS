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

use crate::metadata;
use crate::metadata::error::WzError;
use crate::models::error::ModelError;
use crate::models::item;
use crate::models::item::error::ItemError;
use crate::models::item::model::{
    AndroidEquipType, CashEquipType, EquipType, Inventory, InventoryModel, InventoryTab, ItemModel,
    PetEquipType, RegularEquipType,
};
use crate::models::item::wrapper::Item;
use crate::runtime::state::SharedState;
use serde_json::Value;
use std::collections::HashMap;
use std::time::SystemTime;

pub async fn load_inventory(state: &SharedState, char_id: i32) -> Result<Inventory, ModelError> {
    let items = item::service::get_items_by_char_id(state, char_id).await?;
    let mut equipped_tab: HashMap<i16, Item> = HashMap::new();
    let mut equip_tab: HashMap<i16, Item> = HashMap::new();
    let mut use_tab: HashMap<i16, Item> = HashMap::new();
    let mut setup_tab: HashMap<i16, Item> = HashMap::new();
    let mut etc_tab: HashMap<i16, Item> = HashMap::new();
    let mut cash_tab: HashMap<i16, Item> = HashMap::new();
    for (_, item) in items {
        let pos: i16 = item
            .model
            .pos
            .ok_or(ItemError::NoPos)
            .map_err(ModelError::from)?;
        let equipped: bool = get_equipped_by_pos(pos)?;
        if equipped {
            let equip_type = get_equip_type_from_wz(item.model.wz)?;
            match equip_type {
                EquipType::Android => {
                    let android_equip_type = get_android_equip_type_from_wz(item.model.wz)?;
                    equipped_tab.insert(android_equip_type as i16, item);
                }
                EquipType::Cash => {
                    let cash_equip_type = get_cash_equip_type_from_wz(item.model.wz)?;
                    equipped_tab.insert(cash_equip_type as i16, item);
                }
                EquipType::Pet => {
                    let pet_equip_type = get_pet_equip_type_from_wz(item.model.wz)?;
                    equipped_tab.insert(pet_equip_type as i16, item);
                }
                EquipType::Regular => {
                    let regular_equip_type = get_regular_equip_type_from_wz(item.model.wz)?;
                    equipped_tab.insert(regular_equip_type as i16, item);
                }
            }
        } else {
            let inventory_tab: InventoryTab = get_inventory_tab_from_wz(item.model.wz)?;
            match inventory_tab {
                InventoryTab::Equip => {
                    equip_tab.insert(pos, item);
                }
                InventoryTab::Use => {
                    use_tab.insert(pos, item);
                }
                InventoryTab::Setup => {
                    setup_tab.insert(pos, item);
                }
                InventoryTab::Cash => {
                    cash_tab.insert(pos, item);
                }
                InventoryTab::Etc => {
                    etc_tab.insert(pos, item);
                }
            }
        }
    }
    let inventory: Inventory = Inventory {
        model: InventoryModel { char_id },
        equipped_tab: equipped_tab.clone(),
        equip_tab: equip_tab.clone(),
        use_tab: use_tab.clone(),
        setup_tab: setup_tab.clone(),
        etc_tab: etc_tab.clone(),
        cash_tab: cash_tab.clone(),
    };
    Ok(inventory)
}

pub async fn get_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<HashMap<i32, Item>, ModelError> {
    let mut items: HashMap<i32, Item> = HashMap::new();
    let item_models = item::query::getters::get_item_models_by_char_id(state, char_id).await?;
    for item_model in item_models {
        items.insert(item_model.get_id()?, item_model.load()?);
    }
    Ok(items)
}

pub fn get_item_pos_by_wz(wz: i32) -> Result<i16, ModelError> {
    let equip_type = get_equip_type_from_wz(wz)?;
    let pos: i16 = match equip_type {
        EquipType::Android => {
            let android_equip_type = get_android_equip_type_from_wz(wz)?;
            android_equip_type as i16
        }
        EquipType::Cash => {
            let cash_equip_type = get_cash_equip_type_from_wz(wz)?;
            cash_equip_type as i16
        }
        EquipType::Pet => {
            let pet_equip_type = get_pet_equip_type_from_wz(wz)?;
            pet_equip_type as i16
        }
        EquipType::Regular => {
            let regular_equip_type = get_regular_equip_type_from_wz(wz)?;
            regular_equip_type as i16
        }
    };
    Ok(pos)
}

pub fn get_equipped_by_pos(pos: i16) -> Result<bool, ModelError> {
    let equipped = matches!(
        pos,
        1..=59 | 101..=116 | 201..=206 | 301..=303
    );
    Ok(equipped)
}

pub fn get_inventory_tab_from_wz(wz: i32) -> Result<InventoryTab, ModelError> {
    let filename = String::from("Item.wz");
    let root = metadata::service::get_img_root(wz, &filename)?;
    let map = root.get("info").ok_or(WzError::ObjectError).ok();
    dbg!(map);
    Ok(InventoryTab::Equip)
}

pub fn get_equip_type_from_wz(wz: i32) -> Result<EquipType, ModelError> {
    let filename = String::from("Item.wz");
    let root = metadata::service::get_img_root(wz, &filename)?;
    let map = root.get("info").ok_or(WzError::ObjectError).ok();
    dbg!(map);
    Ok(EquipType::Regular) //placeholder
}

pub fn get_android_equip_type_from_wz(wz: i32) -> Result<AndroidEquipType, ModelError> {
    let filename = String::from("Item.wz");
    let root = metadata::service::get_img_root(wz, &filename)?;
    let map = root.get("info").ok_or(WzError::ObjectError).ok();
    dbg!(map);
    Ok(AndroidEquipType::Hat)
} // placeholder

pub fn get_cash_equip_type_from_wz(wz: i32) -> Result<CashEquipType, ModelError> {
    let filename = String::from("Item.wz");
    let root = metadata::service::get_img_root(wz, &filename)?;
    let map = root.get("info").ok_or(WzError::ObjectError).ok();
    dbg!(map);
    Ok(CashEquipType::Hat)
} // placeholder

pub fn get_pet_equip_type_from_wz(wz: i32) -> Result<PetEquipType, ModelError> {
    let filename = String::from("Item.wz");
    let root = metadata::service::get_img_root(wz, &filename)?;
    let map = root.get("info").ok_or(WzError::ObjectError).ok();
    dbg!(map);
    Ok(PetEquipType::AccOne)
} // placeholder

pub fn get_regular_equip_type_from_wz(wz: i32) -> Result<RegularEquipType, ModelError> {
    let filename = String::from("Item.wz");
    let root = metadata::service::get_img_root(wz, &filename)?;
    let map = root.get("info").ok_or(WzError::ObjectError).ok();
    dbg!(map);
    Ok(RegularEquipType::Hat)
} // placeholder

pub async fn create_item(
    state: &SharedState,
    char_id: Option<i32>,
    pos: Option<i16>,
    item_wz: i32,
) -> Result<Item, ModelError> {
    let filename = String::from("Character.wz");
    let root = metadata::service::get_img_root(item_wz, &filename)?;
    let item_model = ItemModel {
        id: None,
        char_id,
        wz: item_wz,
        pos,
        strength: get_equip_stats_from_wz(&root, "incSTR").unwrap_or(0),
        dexterity: get_equip_stats_from_wz(&root, "incDEX").unwrap_or(0),
        intelligence: get_equip_stats_from_wz(&root, "incINT").unwrap_or(0),
        luck: get_equip_stats_from_wz(&root, "incLUK").unwrap_or(0),
        attack: get_equip_stats_from_wz(&root, "incPAD").unwrap_or(0),
        weapon_defense: get_equip_stats_from_wz(&root, "incPDD").unwrap_or(0),
        magic: get_equip_stats_from_wz(&root, "incMAD").unwrap_or(0),
        magic_defense: get_equip_stats_from_wz(&root, "incMDD").unwrap_or(0),
        hp: get_equip_stats_from_wz(&root, "incMHP").unwrap_or(0),
        mp: get_equip_stats_from_wz(&root, "incMMP").unwrap_or(0),
        accuracy: get_equip_stats_from_wz(&root, "incACC").unwrap_or(0),
        avoid: get_equip_stats_from_wz(&root, "incEVA").unwrap_or(0),
        hands: get_equip_stats_from_wz(&root, "incHANDS").unwrap_or(0),
        speed: get_equip_stats_from_wz(&root, "incSPEED").unwrap_or(0),
        jump: get_equip_stats_from_wz(&root, "incJUMP").unwrap_or(0),
        created_at: Some(SystemTime::now()),
        updated_at: SystemTime::now(),
    };
    item::query::setters::update_items(state, vec![item_model.clone()]).await?;
    Ok(item_model.load()?)
}

fn get_equip_stats_from_wz(root: &Value, key: &str) -> Option<i32> {
    let map = root.get("info").ok_or(WzError::ObjectError).ok()?;
    map.get(key).and_then(|v| v.as_i64().map(|n| n as i32))
}
