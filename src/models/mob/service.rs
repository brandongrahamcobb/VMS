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
use crate::models::mob::model::{MobModel, MobWzInfo, MobWzLife};
use crate::models::mob::wrapper::Mob;

pub fn get_mob_lifes(map_wz: i32) -> Result<Vec<serde_json::Value>, MobError> {
    let mut mob_lifes: Vec<serde_json::Value> = Vec::new();
    let life_json = get_life_json_from_metadata_by_map_wz(map_wz)?;
    let lifes = life_json.as_object().ok_or(MobError::NoLife(map_wz))?;
    for (_, life) in lifes {
        let life_type = life["type"].as_str().ok_or(MobError::NoType)?;
        if life_type == "m" {
            mob_lifes.push(life.clone())
        }
    }
    Ok(mob_lifes)
}

fn get_life_json_from_metadata_by_map_wz(map_wz: i32) -> Result<serde_json::Value, MobError> {
    let filename: String = String::from("Map.wz");
    let json = metadata::service::wz_to_img(map_wz, &filename)?;
    let life = json.get("life").ok_or(MobError::NoLife(map_wz))?.clone();
    Ok(life)
}

pub fn get_mob_wz_life(mob_life: serde_json::Value) -> Result<MobWzLife, MobError> {
    let wz: i32 = mob_life["id"].as_str().unwrap().parse::<i32>().unwrap();
    let x = mob_life["x"].as_i64().unwrap_or(0) as i16;
    let y = mob_life["y"].as_i64().unwrap_or(0) as i16;
    let fh = mob_life["fh"].as_i64().unwrap_or(0) as u16;
    let mob_time = mob_life["mobTime"].as_i64().unwrap_or(0) as u64;
    Ok(MobWzLife {
        wz,
        x,
        y,
        fh,
        mob_time,
    })
}

pub fn get_mob_wz_info(mob_metadata: &MobWzLife) -> Result<MobWzInfo, MobError> {
    let filename: String = String::from("Mob.wz");
    let json = metadata::service::wz_to_img(mob_metadata.wz, &filename)?;
    let info = json["info"].as_object().ok_or(MobError::NoInfo)?;
    let mad: i16 = info["MADamage"].as_i64().unwrap_or(0) as i16;
    let mdd: i16 = info["MDDamage"].as_i64().unwrap_or(0) as i16;
    let pad: i16 = info["PADamage"].as_i64().unwrap_or(0) as i16;
    let pdd: i16 = info["PDDamage"].as_i64().unwrap_or(0) as i16;
    let acc: i16 = info["acc"].as_i64().unwrap_or(0) as i16;
    let body_attack: i8 = info["bodyAttack"].as_i64().unwrap_or(0) as i8;
    let exp: i32 = info["exp"].as_i64().unwrap_or(0) as i32;
    let eva: i16 = info["eva"].as_i64().unwrap_or(0) as i16;
    let fs: f32 = info["fs"].as_i64().unwrap_or(0) as f32;
    let level: i16 = info["level"].as_i64().unwrap_or(0) as i16;
    let max_hp: i32 = info["maxHP"].as_i64().unwrap_or(0) as i32;
    let max_mp: i32 = info["maxMP"].as_i64().unwrap_or(0) as i32;
    let mob_type: i16 = info["mobType"].as_i64().unwrap_or(0) as i16;
    let pushed: i8 = info["pushed"].as_i64().unwrap_or(0) as i8;
    let speed: i16 = info["speed"].as_i64().unwrap_or(0) as i16;
    let summon_type: i16 = info["summonType"].as_i64().unwrap_or(0) as i16;
    let undead: i8 = info["undead"].as_i64().unwrap_or(0) as i8;
    Ok(MobWzInfo {
        mad,
        mdd,
        pad,
        pdd,
        acc,
        body_attack,
        exp,
        eva,
        fs,
        level,
        max_hp,
        max_mp,
        mob_type,
        pushed,
        speed,
        summon_type,
        undead,
    })
}

pub fn init_mob(
    next_id: u32,
    mob_wz_info: &MobWzInfo,
    mob_wz_life: &MobWzLife,
) -> Result<Mob, MobError> {
    let mob_model: MobModel = MobModel {
        id: next_id,
        pos_x: mob_wz_life.x,
        pos_y: mob_wz_life.y,
        hp: mob_wz_info.max_hp,
        mp: mob_wz_info.max_mp,
        fh: mob_wz_life.fh,
        new_state: 0,
        last_x: mob_wz_life.x,
        last_y: mob_wz_life.y,
    };
    let mob: Mob = mob_model.load(mob_wz_info, mob_wz_life)?;
    Ok(mob)
}
