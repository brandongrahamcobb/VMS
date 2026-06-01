/* db/src/skill/model.rs
 * The purpose of this module is to provide a skill model and associated methods.
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

use crate::schema::skills;
use crate::skill::error::SkillModelError;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = skills)]
pub struct SkillModel {
    pub id: Option<i32>,
    pub char_id: i32,
    pub wz: i32,
    pub level: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl SkillModel {
    pub fn get_id(&self) -> Result<i32, SkillModelError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(SkillModelError::NoId)
        }
    }

    pub fn get_created_at(&self) -> Result<SystemTime, SkillModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(SkillModelError::NoCreatedAt(self.get_id()?))
        }
    }
}
