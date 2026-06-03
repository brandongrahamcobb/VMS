/* item/query/getters.rs
 * The purpose of this module is to provide database getters for items.
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
use crate::item::model::{DropData, ItemModel};
use crate::pool::{self, DbPool};
use crate::schema::{drops, items};
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn get_item_models_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<Vec<ItemModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        items::table
            .filter(items::char_id.eq(char_id))
            .get_results::<ItemModel>(conn)
    })
    .await
}

pub async fn get_equipped_item_models_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<Vec<ItemModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        items::table
            .filter(items::char_id.eq(char_id))
            .filter(items::equipped)
            .get_results::<ItemModel>(conn)
    })
    .await
}

pub async fn get_unequipped_item_models_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<Vec<ItemModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        items::table
            .filter(items::char_id.eq(char_id))
            .filter(items::equipped.eq(false))
            .get_results::<ItemModel>(conn)
    })
    .await
}

pub async fn get_item_model_by_item_id(
    pool: &DbPool,
    item_id: i32,
) -> Result<ItemModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        items::table
            .filter(items::id.eq(item_id))
            .get_result::<ItemModel>(conn)
    })
    .await
}

pub async fn get_item_drop_data(
    pool: &DbPool,
    mob_wz: i32,
) -> Result<Vec<DropData>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        drops::table
            .filter(drops::mob_wz.eq(mob_wz))
            .get_results::<DropData>(conn)
    })
    .await
}
