/* account/query/setters.rs
 * The purpose of this module is to provide database setters for accounts.
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
use crate::pool;
use crate::pool::DbPool;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};
use entity::account::model::AccountModel;
use entity::schema::accounts;

pub async fn set_pic_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
    pic: String,
) -> Result<AccountModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(accounts::table.filter(accounts::id.eq(acc_id)))
            .set(accounts::pic.eq(pic))
            .get_result::<AccountModel>(conn)
    })
    .await
}

pub async fn accept_tos_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
) -> Result<AccountModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(accounts::table.filter(accounts::id.eq(acc_id)))
            .set(accounts::accepted_tos.eq(true))
            .get_result::<AccountModel>(conn)
    })
    .await
}

pub async fn update_accounts(
    pool: &DbPool,
    acc_models: Vec<AccountModel>,
) -> Result<Vec<AccountModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        let mut results = Vec::new();
        for acc_model in &acc_models {
            results.push(
                diesel::insert_into(accounts::table)
                    .values(acc_model)
                    .on_conflict(accounts::username)
                    .do_update()
                    .set(acc_model)
                    .get_result::<AccountModel>(conn)?,
            )
        }
        Ok(results)
    })
    .await
}

pub async fn delete_acc_by_id(pool: &DbPool, acc_id: i32) -> Result<usize, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::delete(accounts::table.filter(accounts::id.eq(acc_id))).execute(conn)
    })
    .await
}
