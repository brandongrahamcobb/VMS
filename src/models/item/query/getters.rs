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

use crate::db::schema::{cash_nonequip_items, equip_items, etc_items, setup_items, use_items};
use crate::models::item::cash_nonequip_model::CashNonEquipItemModel;
use crate::models::item::equip_model::EquipItemModel;
use crate::models::item::etc_model::EtcItemModel;
use crate::models::item::setup_model::SetupItemModel;
use crate::models::item::use_model::UseItemModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_equip_item_models_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<Vec<EquipItemModel>> {
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
    equip_items::table
        .filter(equip_items::char_id.eq(char_id))
        .get_results::<EquipItemModel>(&mut conn)
}

pub async fn get_cash_nonequip_item_models_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<Vec<CashNonEquipItemModel>> {
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
    cash_nonequip_items::table
        .filter(cash_nonequip_items::char_id.eq(char_id))
        .get_results::<CashNonEquipItemModel>(&mut conn)
}

pub async fn get_etc_item_models_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<Vec<EtcItemModel>> {
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
    etc_items::table
        .filter(etc_items::char_id.eq(char_id))
        .get_results::<EtcItemModel>(&mut conn)
}

pub async fn get_setup_item_models_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<Vec<SetupItemModel>> {
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
    setup_items::table
        .filter(setup_items::char_id.eq(char_id))
        .get_results::<SetupItemModel>(&mut conn)
}

pub async fn get_use_item_models_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<Vec<UseItemModel>> {
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
    use_items::table
        .filter(use_items::char_id.eq(char_id))
        .get_results::<UseItemModel>(&mut conn)
}
