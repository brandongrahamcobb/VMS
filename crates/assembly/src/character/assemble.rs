/* assembly/src/character/assemble.rs
 * The purpose of this module is to assemble a character wrapper.
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

use crate::character::error::CharacterAssemblyError;
use crate::{item, keybinding, skill};
use db;
use db::pool::DbPool;
use entity::character::model::CharacterModel;
use entity::character::wrapper::Character;
use entity::item::wrapper::Inventory;
use entity::job::model::JobModel;
use entity::job::wrapper::{Job, JobWzInfo};
use entity::keybinding::wrapper::Keybinding;
use entity::map::model::Point;
use entity::skill::wrapper::Skill;
use metadata;
use std::collections::HashMap;

pub async fn assemble_char_by_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<Character, CharacterAssemblyError> {
    let char_model: CharacterModel =
        db::character::getters::get_char_model_by_id(pool, char_id).await?;
    let binds: HashMap<i32, Keybinding> =
        keybinding::assemble::assemble_keybindings_by_char_id(pool, char_id).await?;
    let inventory: Inventory = item::assemble::assemble_inventory_by_char_id(pool, char_id).await?;
    let job: Job = Job {
        model: JobModel,
        info: JobWzInfo {
            skills: metadata::job::skill::get_job_skills_by_job_wz(char_model.job_wz)?,
            wz: char_model.job_wz,
        },
    };
    let skills: HashMap<i32, Skill> =
        skill::assemble::assemble_skills_by_char_id(pool, char_id).await?;
    let pos: Point = metadata::map::portal::get_zeroeth_portal_spawnpoint(char_model.map_wz)?;
    Ok(Character {
        model: char_model,
        binds,
        inventory,
        job,
        skills,
        pos,
    })
}
