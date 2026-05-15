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
use crate::models::error::ModelError;
use crate::models::mob::wrapper::Mob;

pub fn load_mobs(map_wz: i32) -> Result<HashMap<u32, Mob>, ModelError> {
    let root = metadata::service::get_img_root(map_wz, "Mob.wz")?;
    dbg!(root);
    Ok(HashMap::new())
}
