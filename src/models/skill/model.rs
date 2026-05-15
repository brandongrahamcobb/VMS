/* skill/model.rs
 * The purpose of this module is to provide a skill model.
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

use crate::db::schema::skills;
use crate::models::error::ModelError;
use crate::models::skill::wrapper::Skill;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = skills)]
pub struct SkillModel {
    pub char_id: i32,
    pub wz: i32,
    pub level: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl SkillModel {
    pub fn load(&self) -> Result<Skill, ModelError> {
        Ok(Skill {
            model: self.clone(),
        })
    }
}
