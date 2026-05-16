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

use std::collections::HashMap;

use crate::metadata;
use crate::models::mob::error::MobError;
use crate::models::mob::model::MobModel;
use crate::models::mob::wrapper::Mob;

pub fn load_mobs(map_wz: i32) -> Result<(HashMap<u32, Mob>, u32), MobError> {
    let mut mobs: HashMap<u32, Mob> = HashMap::new();
    let mut next_id: u32 = 1;
    let filename: String = String::from("Map.wz");
    let json = metadata::service::wz_to_img(map_wz, &filename)?;
    let filename: String = String::from("Mob.wz");
    if let Some(life) = json["life"].as_object() {
        for (_, value) in life {
            let x = value["x"].as_i64().unwrap_or(0) as i16;
            let y = value["y"].as_i64().unwrap_or(0) as i16;
            let fh = value["fh"].as_i64().unwrap_or(0) as i16;
            if let Some(m) = value["type"].as_str() {
                if m == "m" {
                    let id: i32 = value["id"].as_str().unwrap().parse::<i32>().unwrap();
                    let json = metadata::service::wz_to_img(id, &filename)?;
                    if let Some(info) = json["info"].as_object() {
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
                        let mob_model: MobModel = MobModel {
                            id: next_id,
                            wz: id,
                            pos_x: x,
                            pos_y: y,
                            fh,
                            hp: max_hp,
                            level,
                            max_hp,
                            mp: max_mp,
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
                        };
                        mobs.insert(next_id, mob_model.load()?);
                        next_id += 1;
                    }
                }
            }
        }
    }
    Ok((mobs, next_id))
}
