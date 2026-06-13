/* db/src/keybinding/setters.rs
 * The purpose of this module is to provide database setters for keybindings.
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
use crate::keybinding::model::KeybindingModel;
use crate::pool::{self, DbPool};
use crate::schema::keybindings;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn update_keybindings(
    pool: &DbPool,
    bind_models: Vec<KeybindingModel>,
) -> Result<Vec<KeybindingModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        let mut results: Vec<KeybindingModel> = Vec::new();
        for bind_model in &bind_models {
            results.push(
                diesel::insert_into(keybindings::table)
                    .values(bind_model)
                    .on_conflict((keybindings::char_id, keybindings::key))
                    .do_update()
                    .set(bind_model)
                    .get_result::<KeybindingModel>(conn)?,
            )
        }
        Ok(results)
    })
    .await
}

pub async fn delete_keybinding_by_id(pool: &DbPool, bind_id: i32) -> Result<usize, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::delete(keybindings::table.filter(keybindings::id.eq(bind_id))).execute(conn)
    })
    .await
}
