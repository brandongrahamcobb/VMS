// /* mob/service.rs
//  * The purpose of this module is to provide assisting functions and implementations for mobs.
//  *
//  * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
//  *
//  * This program is free software: you can redistribute it and/or modify
//  * it under the terms of the GNU Affero General Public License as published by
//  * the Free Software Foundation, either version 3 of the License, or
//  * (at your option) any later version.
//  *
//  * This program is distributed in the hope that it will be useful,
//  * but WITHOUT ANY WARRANTY; without even the implied warranty of
//  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  * GNU Affero General Public License for more details.
//  *
//  * You should have received a copy of the GNU Affero General Public License
//  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
//  */
//
// use crate::models::error::ModelError;
// use crate::models::mob::model::MobModel;
// use crate::wz;
// use crate::wz::error::WzError;
//
// pub fn get_mob_models_by_map_wz(map_wz: i32) -> Result<Vec<MobModel>, ModelError> {
//     let root = wz::service::get_img_root(map_wz, "Map.wz")?;
//     let mut mob_ids: Vec<i32> = Vec::new();
//     for (_key, target) in wz_life {
//         let life_type = target.get("type").and_then(|v| v.as_str()).unwrap_or("");
//         if life_type != "m" {
//             continue;
//         }
//         let mob_id = target
//             .get("id")
//             .and_then(|v| v.as_str())
//             .and_then(|s| s.parse::<i32>().ok())
//             .ok_or(WzError::ObjectError)?;
//         mob_ids.push(mob_id);
//     }
//     Ok(())
// }
