/* db/src/keybinding/getters.rs
 * The purpose of this module is to provide database getters for keybindings.
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
use entity::keybinding::model::KeybindingModel;
use entity::schema::keybindings;

pub async fn get_keybinding_models_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<Vec<KeybindingModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        keybindings::table
            .filter(keybindings::char_id.eq(char_id))
            .load::<KeybindingModel>(conn)
    })
    .await
}

pub async fn get_keybinding_model_by_id(
    pool: &DbPool,
    bind_id: i32,
) -> Result<KeybindingModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        keybindings::table
            .filter(keybindings::id.eq(bind_id))
            .get_result::<KeybindingModel>(conn)
    })
    .await
}
