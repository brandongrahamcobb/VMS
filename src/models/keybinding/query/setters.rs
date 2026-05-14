/* keybinding/query/setters.rs
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

use crate::db::schema::keybindings;
use crate::models::keybinding::model::KeybindingModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn update_keybindings(
    state: &SharedState,
    bind_models: Vec<KeybindingModel>,
) -> QueryResult<Vec<KeybindingModel>> {
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
    for bind_model in &bind_models {
        results.push(
            diesel::insert_into(keybindings::table)
                .values(bind_model)
                .on_conflict((keybindings::char_id, keybindings::key))
                .do_update()
                .set(bind_model)
                .get_result::<KeybindingModel>(&mut conn)?,
        )
    }
    Ok(results)
}

pub async fn delete_keybindings_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<usize> {
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
    diesel::delete(keybindings::table.filter(keybindings::char_id.eq(char_id))).execute(&mut conn)
}
