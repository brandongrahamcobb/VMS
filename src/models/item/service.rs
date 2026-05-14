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

use crate::models::error::ModelError;
use crate::models::item;
use crate::models::item::model::{EquipType, ItemModel, RegularEquipType};
use crate::models::item::wrapper::Item;
use crate::runtime::state::SharedState;
use crate::wz;
use crate::wz::error::WzError;
use serde_json::Value;
use std::time::SystemTime;

pub async fn get_items_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Item>, ModelError> {
    let mut items: Vec<Item> = Vec::<Item>::new();
    let item_models = item::query::getters::get_item_models_by_char_id(state, char_id).await?;
    for item_model in item_models {
        items.push(item_model.load()?)
    }
    Ok(items)
}

pub fn get_equip_type_from_wz(wz_id: i32) -> Result<EquipType, ModelError> {
    let category = wz::service::get_img_root(wz_id, "Item.wz");
    dbg!(category);
    Ok(EquipType::RegularEquipType(RegularEquipType::Hat))
}

pub async fn create_item(
    state: &SharedState,
    char_id: Option<i32>,
    equipped: bool,
    item_wz: i32,
) -> Result<Item, ModelError> {
    let filename = String::from("Character.wz");
    let root = wz::service::get_img_root(item_wz, &filename)?;
    let item_model = ItemModel {
        id: None,
        char_id,
        equipped,
        wz: item_wz,
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
