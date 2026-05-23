/* assembly/src/mob/assemble.rs
 * The purpose of this module is to assemble a mob wrapper.
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

use entity::mob::model::{LifeState, MobModel, MobWzInfo, MobWzLife};
use entity::mob::wrapper::Mob;
use std::collections::HashMap;

use crate::mob::error::MobAssemblyError;

pub fn assemble_mobs_by_map_wz(map_wz: i32) -> Result<HashMap<u32, Mob>, MobAssemblyError> {
    let mut mobs: HashMap<u32, Mob> = HashMap::new();
    let mut next_id: u32 = 1;
    let mob_lifes = metadata::map::mob::get_mob_lifes(map_wz)?;
    for mob_life in mob_lifes {
        let mob_wz_life: MobWzLife = metadata::map::mob::get_mob_wz_life(mob_life.clone())?;
        let mob_wz_info: MobWzInfo = metadata::map::mob::get_mob_wz_info(&mob_wz_life)?;
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
        let mob: Mob = Mob {
            model: mob_model,
            info: mob_wz_info,
            life: mob_wz_life,
            life_state: LifeState::Alive,
        };
        mobs.insert(next_id, mob);
        next_id += 1;
    }
    Ok(mobs)
}
