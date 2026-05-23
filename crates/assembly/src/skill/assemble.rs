/* assembly/src/skill/assemble.rs
 * The purpose of this module is to assemble skills.
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

use db;
use db::pool::DbPool;
use entity::skill::wrapper::Skill;
use std::collections::HashMap;

use crate::skill::error::SkillAssemblyError;

pub async fn assemble_skills_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<HashMap<i32, Skill>, SkillAssemblyError> {
    let skill_models = db::skill::getters::get_skill_models_by_char_id(pool, char_id).await?;
    let mut skills: HashMap<i32, Skill> = HashMap::new();
    for skill_model in skill_models {
        skills.insert(skill_model.get_id()?, Skill { model: skill_model });
    }
    Ok(skills)
}

pub async fn assemble_skill_by_id(
    pool: &DbPool,
    skill_id: i32,
) -> Result<Skill, SkillAssemblyError> {
    let skill_model = db::skill::getters::get_skill_model_by_id(pool, skill_id).await?;
    let skill = Skill { model: skill_model };
    Ok(skill)
}
