/* skills/service.rs
 * The purpose of this module is to provide assisting functions and implementations for skills.
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

use crate::db::error::DatabaseError;
use crate::models::skill;
use crate::models::skill::error::SkillError;
use crate::models::skill::wrapper::Skill;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

pub async fn load_skills(
    state: &SharedState,
    char_id: i32,
) -> Result<HashMap<i32, Skill>, SkillError> {
    let skill_models = skill::query::getters::get_skill_models_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    Ok(skill_models
        .into_iter()
        .map(|s| -> Result<(i32, Skill), SkillError> { Ok((s.wz, s.load()?)) })
        .collect::<Result<HashMap<i32, Skill>, SkillError>>()?)
}
