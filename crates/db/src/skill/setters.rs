/* skill/query/setters.rs
 * The purpose of this module is to provide database setters for skills.
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

use crate::error::DatabaseError;
use crate::pool::{self, DbPool};
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};
use crate::schema::skills;
use crate::skill::model::SkillModel;

pub async fn update_skills(
    pool: &DbPool,
    skill_models: Vec<SkillModel>,
) -> Result<Vec<SkillModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        let mut results = Vec::new();
        for skill_model in &skill_models {
            results.push(
                diesel::insert_into(skills::table)
                    .values(skill_model)
                    .on_conflict((skills::char_id, skills::wz))
                    .do_update()
                    .set(skill_model)
                    .get_result::<SkillModel>(conn)?,
            )
        }
        Ok(results)
    })
    .await
}

pub async fn delete_skill_by_id(pool: &DbPool, skill_id: i32) -> Result<usize, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::delete(skills::table.filter(skills::id.eq(skill_id))).execute(conn)
    })
    .await
}
