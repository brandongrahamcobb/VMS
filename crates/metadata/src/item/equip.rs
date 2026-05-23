/* metadata/src/item/equip.rs
 * The purpose of this module is to provide metadata access to equips.
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
use crate::item::inventory;
use crate::service;
use entity::item::model::{CASH_EQUIP_SLOTS, InventoryTab, ItemWzInfo, OTHER_EQUIP_SLOTS};
use serde_json::Value;

fn get_equip_stats_by_wz(root: &Value, key: &str) -> Result<Option<i16>, ItemMetadataError> {
    let map = root.get("info").ok_or(ItemMetadataError::EquipError)?;
    Ok(map.get(key).and_then(|v| v.as_i64().map(|n| n as i16)))
}

pub fn build_equip_item_wz_info_by_wz(wz: i32) -> Result<ItemWzInfo, ItemMetadataError> {
    let itab: InventoryTab = inventory::get_inventory_tab_by_wz(wz)?;
    let filename: String = String::from("Character.wz");
    let json = service::wz_to_img(wz, &filename)?;
    let islot = json["info"]["islot"]
        .as_str()
        .ok_or(ItemMetadataError::EquipError)?
        .to_string();
    let cash = json["info"]["cash"] == 1;
    let wz_info = ItemWzInfo {
        cash,
        islot: Some(islot),
        itab: itab as i8,
        strength: get_equip_stats_by_wz(&json, "incSTR")?.unwrap_or(0),
        dexterity: get_equip_stats_by_wz(&json, "incDEX")?.unwrap_or(0),
        intelligence: get_equip_stats_by_wz(&json, "incINT")?.unwrap_or(0),
        luck: get_equip_stats_by_wz(&json, "incLUK")?.unwrap_or(0),
        attack: get_equip_stats_by_wz(&json, "incPAD")?.unwrap_or(0),
        weapon_defense: get_equip_stats_by_wz(&json, "incPDD")?.unwrap_or(0),
        magic: get_equip_stats_by_wz(&json, "incMAD")?.unwrap_or(0),
        magic_defense: get_equip_stats_by_wz(&json, "incMDD")?.unwrap_or(0),
        hp: get_equip_stats_by_wz(&json, "incMHP")?.unwrap_or(0),
        mp: get_equip_stats_by_wz(&json, "incMMP")?.unwrap_or(0),
        accuracy: get_equip_stats_by_wz(&json, "incACC")?.unwrap_or(0),
        avoid: get_equip_stats_by_wz(&json, "incEVA")?.unwrap_or(0),
        hands: get_equip_stats_by_wz(&json, "incHANDS")?.unwrap_or(0),
        speed: get_equip_stats_by_wz(&json, "incSPEED")?.unwrap_or(0),
        jump: get_equip_stats_by_wz(&json, "incJUMP")?.unwrap_or(0),
        slots: 0, // placeholder
        flag: 0,  //placeholder
    };
    Ok(wz_info)
}

pub fn get_equip_ipos_by_wz(wz: i32) -> Result<i16, ItemMetadataError> {
    let filename: String = String::from("Character.wz");
    let json = service::wz_to_img(wz, &filename)?;
    let islot = json["info"]["islot"]
        .as_str()
        .ok_or(ItemMetadataError::EquipError)?;
    let cash = json["info"]["cash"]
        .as_i64()
        .ok_or(ItemMetadataError::EquipError)?;
    if cash == 0 {
        return OTHER_EQUIP_SLOTS
            .iter()
            .flat_map(|group| group.iter())
            .find(|slot| slot.islot == islot)
            .map(|slot| slot.key)
            .ok_or(ItemMetadataError::EquipError);
    } else {
        return CASH_EQUIP_SLOTS
            .iter()
            .find(|slot| slot.islot == islot)
            .map(|slot| slot.key)
            .ok_or(ItemMetadataError::EquipError);
    }
}
