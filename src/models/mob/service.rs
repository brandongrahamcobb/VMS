/* mob/service.rs
 * The purpose of this module is to provide assisting functions for mobs.
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
use crate::models::mob::error::MobError;
use crate::models::mob::model::MobModel;
use std::collections::HashMap;

pub fn get_mob_models(map_wz: i32) -> Result<HashMap<u32, MobModel>, MobError> {
    let mut mob_models: HashMap<u32, MobModel> = HashMap::new();
    let mut next_id: u32 = 1;
    let mob_lifes = get_mob_lifes(map_wz)?;
    for mob_life in mob_lifes {
        let mob_metadata = get_mob_metadata(mob_life.clone())?;
        let mob_info = get_mob_info(mob_life.clone())?;
        let mob_model = build_mob(next_id, &mob_metadata, &mob_info)?;
        mob_models.insert(next_id, mob_model);
        next_id += 1;
    }
    Ok(mob_models)
}

pub fn get_mob_lifes(map_wz: i32) -> Result<Vec<serde_json::Value>, MobError> {
    let mut mob_lifes: Vec<serde_json::Value> = Vec::new();
    let life_map = get_life_json_map_from_metadata_by_map_wz(map_wz)?;
    for (_, life) in life_map {
        mob_lifes.push(life);
    }
    Ok(mob_lifes)
}

fn get_life_json_map_from_metadata_by_map_wz(
    map_wz: i32,
) -> Result<serde_json::Map<String, serde_json::Value>, MobError> {
    let filename: String = String::from("Map.wz");
    let json = metadata::service::wz_to_img(map_wz, &filename)?;
    let life_map = json["life"].as_object().ok_or(MobError::NoLife)?;
    Ok(life_map.clone())
}

struct MobMetadata {
    wz: i32,
    x: i16,
    y: i16,
    fh: i16,
    mob_time: u64,
}

fn get_mob_metadata(life_metadata: serde_json::Value) -> Result<MobMetadata, MobError> {
    let life_type = life_metadata["type"].as_str().ok_or(MobError::NoType)?;
    if life_type == "m" {
        let wz: i32 = life_metadata["id"]
            .as_str()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let x = life_metadata["x"].as_i64().unwrap_or(0) as i16;
        let y = life_metadata["y"].as_i64().unwrap_or(0) as i16;
        let fh = life_metadata["fh"].as_i64().unwrap_or(0) as i16;
        let mob_time = life_metadata["mobTime"].as_i64().unwrap_or(0) as u64;
        Ok(MobMetadata {
            wz,
            x,
            y,
            fh,
            mob_time,
        })
    } else {
        Err(MobError::NotMob)
    }
}

struct MobInfo {
    level: i16,
    max_hp: i32,
    max_mp: i32,
    exp: i32,
    pad: i16,
    mad: i16,
    pdd: i16,
    mdd: i16,
    acc: i16,
    eva: i16,
    speed: i16,
    undead: i8,
    body_attack: i8,
    pushed: i8,
}

fn get_mob_info(life: serde_json::Value) -> Result<MobInfo, MobError> {
    let info = life["info"].as_object().ok_or(MobError::NoInfo)?;
    let level = info["level"].as_i64().unwrap_or(0) as i16;
    let max_hp = info["maxHP"].as_i64().unwrap_or(0) as i32;
    let max_mp = info["maxMP"].as_i64().unwrap_or(0) as i32;
    let exp = info["exp"].as_i64().unwrap_or(0) as i32;
    let pad = info["PADamage"].as_i64().unwrap_or(0) as i16;
    let mad = info["MADamage"].as_i64().unwrap_or(0) as i16;
    let pdd = info["PDDamage"].as_i64().unwrap_or(0) as i16;
    let mdd = info["MDDamage"].as_i64().unwrap_or(0) as i16;
    let acc = info["acc"].as_i64().unwrap_or(0) as i16;
    let eva = info["eva"].as_i64().unwrap_or(0) as i16;
    let speed = info["speed"].as_i64().unwrap_or(0) as i16;
    let undead = info["undead"].as_i64().unwrap_or(0) as i8;
    let body_attack = info["bodyAttack"].as_i64().unwrap_or(0) as i8;
    let pushed = info["pushed"].as_i64().unwrap_or(0) as i8;
    Ok(MobInfo {
        level,
        max_hp,
        max_mp,
        exp,
        pad,
        mad,
        pdd,
        mdd,
        acc,
        eva,
        speed,
        undead,
        body_attack,
        pushed,
    })
}

fn build_mob(
    next_id: u32,
    mob_metadata: &MobMetadata,
    mob_info: &MobInfo,
) -> Result<MobModel, MobError> {
    let mob_model: MobModel = MobModel {
        id: next_id,
        wz: mob_metadata.wz,
        pos_x: mob_metadata.x,
        pos_y: mob_metadata.y,
        fh: mob_metadata.fh,
        mob_time: mob_metadata.mob_time,
        hp: mob_info.max_hp,
        level: mob_info.level,
        max_hp: mob_info.max_hp,
        mp: mob_info.max_mp,
        max_mp: mob_info.max_mp,
        exp: mob_info.exp,
        pad: mob_info.pad,
        mad: mob_info.mad,
        pdd: mob_info.pdd,
        mdd: mob_info.mdd,
        acc: mob_info.acc,
        eva: mob_info.eva,
        speed: mob_info.speed,
        undead: mob_info.undead,
        body_attack: mob_info.body_attack,
        pushed: mob_info.pushed,
    };
    Ok(mob_model)
}
