/* db/src/character/setters.rs
 * The purpose of this module is to provide database setters for characters.
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

use crate::character::model::CharacterModel;
use crate::error::DatabaseError;
use crate::pool::{self, DbPool};
use crate::schema::characters;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn update_characters(
    pool: &DbPool,
    char_models: Vec<CharacterModel>,
) -> Result<Vec<CharacterModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        let mut results = Vec::new();
        for char_model in &char_models {
            results.push(
                diesel::insert_into(characters::table)
                    .values(char_model)
                    .on_conflict(characters::ign)
                    .do_update()
                    .set(char_model)
                    .get_result::<CharacterModel>(conn)?,
            )
        }
        Ok(results)
    })
    .await
}

pub async fn delete_char_by_id(pool: &DbPool, char_id: i32) -> Result<usize, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::delete(characters::table.filter(characters::id.eq(char_id))).execute(conn)
    })
    .await
}
