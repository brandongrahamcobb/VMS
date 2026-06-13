/* db/src/item/setters.rs
 * The purpose of this module is to provide database setters for items.
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
use crate::item::model::ItemModel;
use crate::pool::{self, DbPool};
use crate::schema::items;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn delete_item_by_id(pool: &DbPool, item_id: i32) -> Result<usize, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::delete(items::table.filter(items::id.eq(item_id))).execute(conn)
    })
    .await
}

pub async fn update_items(
    pool: &DbPool,
    item_models: Vec<ItemModel>,
) -> Result<Vec<ItemModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        let mut results: Vec<ItemModel> = Vec::new();
        for item_model in &item_models {
            results.push(
                diesel::insert_into(items::table)
                    .values(item_model)
                    .on_conflict(items::id)
                    .do_update()
                    .set(item_model)
                    .get_result::<ItemModel>(conn)?,
            )
        }
        Ok(results)
    })
    .await
}
