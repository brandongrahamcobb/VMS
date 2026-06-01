/* skill/query/getters.rs
 * The purpose of this module is to provide database getters for skills.
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

pub async fn get_skill_model_by_character_id_and_skill_id(
    pool: &DbPool,
    char_id: i32,
    wz: i32,
) -> Result<SkillModel, DatabaseError> {
    let result: SkillModel = pool::spawn_db(pool, move |conn| {
        skills::table
            .filter(skills::char_id.eq(char_id))
            .filter(skills::wz.eq(wz))
            .first::<SkillModel>(conn)
    })
    .await?;
    Ok(result)
}

pub async fn get_skill_models_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<Vec<SkillModel>, DatabaseError> {
    let results: Vec<SkillModel> = pool::spawn_db(pool, move |conn| {
        skills::table
            .filter(skills::char_id.eq(char_id))
            .load::<SkillModel>(conn)
    })
    .await?;
    Ok(results)
}

pub async fn get_skill_model_by_id(
    pool: &DbPool,
    skill_id: i32,
) -> Result<SkillModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        skills::table
            .filter(skills::id.eq(skill_id))
            .get_result::<SkillModel>(conn)
    })
    .await
}
