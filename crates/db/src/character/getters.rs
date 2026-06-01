/* character/query/getters.rs
 * The purpose of this module is to provide database getters for characters.
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
use crate::schema::{character_limits, characters};
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn get_char_models_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
) -> Result<Vec<CharacterModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        characters::table
            .filter(characters::acc_id.eq(acc_id))
            .load::<CharacterModel>(conn)
    })
    .await
}

pub async fn get_char_models_by_acc_id_and_world_id(
    pool: &DbPool,
    acc_id: i32,
    world_id: i16,
) -> Result<Vec<CharacterModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        characters::table
            .filter(characters::acc_id.eq(acc_id))
            .filter(characters::world_id.eq(world_id))
            .load::<CharacterModel>(conn)
    })
    .await
}

pub async fn get_char_model_by_name(
    pool: &DbPool,
    ign: String,
) -> Result<CharacterModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        characters::table
            .filter(characters::ign.eq(ign))
            .first::<CharacterModel>(conn)
    })
    .await
}

pub async fn get_char_model_by_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<CharacterModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        characters::table
            .filter(characters::id.eq(char_id))
            .first::<CharacterModel>(conn)
    })
    .await
}

pub async fn get_char_max_by_account_and_world_id(
    pool: &DbPool,
    acc_id: i32,
    world_id: i16,
) -> Result<i16, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        character_limits::table
            .filter(&character_limits::acc_id.eq(acc_id))
            .filter(&character_limits::world_id.eq(world_id))
            .select(&character_limits::char_max)
            .first(conn)
    })
    .await
}
