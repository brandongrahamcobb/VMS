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

use crate::db::schema::skills;
use crate::models::character::skill::model::SkillModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn update_skills(
    state: &SharedState,
    skill_models: Vec<SkillModel>,
) -> QueryResult<Vec<SkillModel>> {
    let db = {
        let state = state.lock().await;
        state.db.clone()
    };
    let mut conn = db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    let mut results = Vec::new();
    for skill_model in &skill_models {
        results.push(
            diesel::insert_into(skills::table)
                .values(skill_model)
                .on_conflict((skills::char_id, skills::wz))
                .do_update()
                .set(skill_model)
                .get_result::<SkillModel>(&mut conn)?,
        )
    }
    Ok(results)
}

pub async fn delete_skills_by_char_id(state: &SharedState, char_id: i32) -> QueryResult<usize> {
    let db = {
        let state = state.lock().await;
        state.db.clone()
    };
    let mut conn = db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    diesel::delete(skills::table.filter(skills::char_id.eq(char_id))).execute(&mut conn)
}
