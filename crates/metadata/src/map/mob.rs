/* metadata/src/map/mob.rs
 * The purpose of this module is to provide metadata access to mobs.
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

use crate::map::error::MapMetadataError;
use crate::service;
use base::mob::BaseMob;

pub fn get_mob_rate_by_map_wz(map_wz: i32) -> Result<f32, MapMetadataError> {
    let filename: &str = "Map.wz";
    let json = service::wz_to_img(map_wz, filename)?;
    let mob_rate = json["info"]["mobRate"]
        .as_f64()
        .ok_or(MapMetadataError::MobError)? as f32;
    Ok(mob_rate)
}

pub fn get_mob_lifes(map_wz: i32) -> Result<Vec<serde_json::Value>, MapMetadataError> {
    let mut mob_lifes: Vec<serde_json::Value> = Vec::new();
    let life_json = get_life_json_from_metadata_by_map_wz(map_wz)?;
    let lifes = life_json.as_object().ok_or(MapMetadataError::MobError)?;
    for (_, life) in lifes {
        let life_type = life["type"].as_str().ok_or(MapMetadataError::MobError)?;
        if life_type == "m" {
            mob_lifes.push(life.clone())
        }
    }
    Ok(mob_lifes)
}

fn get_life_json_from_metadata_by_map_wz(
    map_wz: i32,
) -> Result<serde_json::Value, MapMetadataError> {
    let filename: String = String::from("Map.wz");
    let json = service::wz_to_img(map_wz, &filename)?;
    let life = json.get("life").ok_or(MapMetadataError::MobError)?.clone();
    Ok(life)
}

pub fn get_base_mobs_by_map_wz(map_wz: i32) -> Result<Vec<BaseMob>, MapMetadataError> {
    let mut base_mobs: Vec<BaseMob> = Vec::new();
    let mob_lifes: Vec<serde_json::Value> = get_mob_lifes(map_wz)?;
    for mob_life in mob_lifes {
        let wz: i32 = mob_life["id"]
            .as_str()
            .ok_or(MapMetadataError::MobError)?
            .parse::<i32>()
            .map_err(MapMetadataError::from)?;
        let x = mob_life["x"].as_i64().unwrap_or(0) as i16;
        let y = mob_life["y"].as_i64().unwrap_or(0) as i16;
        let fh = mob_life["fh"].as_i64().unwrap_or(0) as u16;
        let mob_time = mob_life["mobTime"].as_i64().unwrap_or(0) as u64;
        let filename: String = String::from("Mob.wz");
        let json = service::wz_to_img(wz, &filename)?;
        let info = json["info"].as_object().ok_or(MapMetadataError::MobError)?;
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
        let max_mp: i16 = info["maxMP"].as_i64().unwrap_or(0) as i16;
        let mob_type: i16 = info["mobType"].as_i64().unwrap_or(0) as i16;
        let pushed: i8 = info["pushed"].as_i64().unwrap_or(0) as i8;
        let speed: i16 = info["speed"].as_i64().unwrap_or(0) as i16;
        let summon_type: i16 = info["summonType"].as_i64().unwrap_or(0) as i16;
        let undead: i8 = info["undead"].as_i64().unwrap_or(0) as i8;
        base_mobs.push(BaseMob {
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
            fh,
            mob_time,
            wz,
            x,
            y,
        });
    }
    Ok(base_mobs)
}
