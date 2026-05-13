/* account/query/getters.rs
 * The purpose of this module is to provide database getters for accounts.
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

use crate::db::schema::accounts;
use crate::models::account::model::AccountModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_account_model_by_username(
    state: &SharedState,
    user: String,
) -> QueryResult<AccountModel> {
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
    accounts::table
        .filter(accounts::username.eq(user))
        .first::<AccountModel>(&mut conn)
}

pub async fn get_account_models_by_channel_world_ids_map_wz(
    state: &SharedState,
    channel_id: i16,
    map_wz: i32,
    world_id: i16,
) -> QueryResult<Vec<AccountModel>> {
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
    accounts::table
        .filter(accounts::world_id.eq(world_id))
        .filter(accounts::channel_id.eq(channel_id))
        .filter(accounts::map_wz.eq(map_wz))
        .get_results::<AccountModel>(&mut conn)
}

pub async fn get_account_models_by_channel_world_ids(
    state: &SharedState,
    channel_id: i16,
    world_id: i16,
) -> QueryResult<Vec<AccountModel>> {
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
    accounts::table
        .filter(accounts::world_id.eq(world_id))
        .filter(accounts::channel_id.eq(channel_id))
        .get_results::<AccountModel>(&mut conn)
}

pub async fn get_account_models_by_map_wz_world_id(
    state: &SharedState,
    map_wz: i32,
    world_id: i16,
) -> QueryResult<Vec<AccountModel>> {
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
    accounts::table
        .filter(accounts::world_id.eq(world_id))
        .filter(accounts::map_wz.eq(map_wz))
        .get_results::<AccountModel>(&mut conn)
}

pub async fn get_account_models_by_map_wz(
    state: &SharedState,
    map_wz: i32,
) -> QueryResult<Vec<AccountModel>> {
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
    accounts::table
        .filter(accounts::map_wz.eq(map_wz))
        .get_results::<AccountModel>(&mut conn)
}

pub async fn get_account_models_by_channel_id_map_wz(
    state: &SharedState,
    channel_id: i16,
    world_id: i16,
) -> QueryResult<Vec<AccountModel>> {
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
    accounts::table
        .filter(accounts::world_id.eq(world_id))
        .filter(accounts::channel_id.eq(channel_id))
        .get_results::<AccountModel>(&mut conn)
}

pub async fn get_account_models_by_channel_id(
    state: &SharedState,
    channel_id: i16,
) -> QueryResult<Vec<AccountModel>> {
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
    accounts::table
        .filter(accounts::channel_id.eq(channel_id))
        .get_results::<AccountModel>(&mut conn)
}

pub async fn get_account_models_by_world_id(
    state: &SharedState,
    world_id: i16,
) -> QueryResult<Vec<AccountModel>> {
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
    accounts::table
        .filter(accounts::world_id.eq(world_id))
        .get_results::<AccountModel>(&mut conn)
}
