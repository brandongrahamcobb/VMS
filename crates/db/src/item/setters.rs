/* item/query/setters.rs
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
use crate::pool::{self, DbPool};
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};
use crate::item::model::ItemModel;
use crate::schema::items;

pub async fn delete_item_by_id(pool: &DbPool, item_id: i32) -> Result<usize, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::delete(items::table.filter(items::id.eq(item_id))).execute(conn)
    })
    .await
}

pub async fn update_item(
    pool: &DbPool,
    item_model: &ItemModel,
) -> Result<ItemModel, DatabaseError> {
    let item_model: ItemModel = item_model.clone();
    pool::spawn_db(pool, move |conn| {
        diesel::insert_into(items::table)
            .values(item_model.clone())
            .on_conflict(items::id)
            .do_update()
            .set(item_model.clone())
            .get_result::<ItemModel>(conn)
    })
    .await
}
