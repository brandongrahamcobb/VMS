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

use crate::db::schema::{cash_nonequip_items, equip_items, etc_items, setup_items, use_items};
use crate::models::item::cash_nonequip_model::CashNonEquipItemModel;
use crate::models::item::equip_model::EquipItemModel;
use crate::models::item::etc_model::EtcItemModel;
use crate::models::item::setup_model::SetupItemModel;
use crate::models::item::use_model::UseItemModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn delete_items_by_char_id(state: &SharedState, char_id: i32) -> QueryResult<usize> {
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
    let mut deleted: usize = 0;
    deleted +=
        diesel::delete(cash_nonequip_items::table.filter(cash_nonequip_items::char_id.eq(char_id)))
            .execute(&mut conn)?;
    deleted += diesel::delete(equip_items::table.filter(equip_items::char_id.eq(char_id)))
        .execute(&mut conn)?;
    deleted += diesel::delete(etc_items::table.filter(etc_items::char_id.eq(char_id)))
        .execute(&mut conn)?;
    deleted += diesel::delete(setup_items::table.filter(setup_items::char_id.eq(char_id)))
        .execute(&mut conn)?;
    deleted += diesel::delete(use_items::table.filter(use_items::char_id.eq(char_id)))
        .execute(&mut conn)?;
    Ok(deleted)
}

impl EquipItemModel {
    pub async fn update_item(&self, state: &SharedState) -> QueryResult<Self> {
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
        diesel::insert_into(equip_items::table)
            .values(self.clone())
            .on_conflict(equip_items::id)
            .do_update()
            .set(self.clone())
            .get_result::<Self>(&mut conn)
    }
}

impl CashNonEquipItemModel {
    pub async fn update_item(&self, state: &SharedState) -> QueryResult<Self> {
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
        diesel::insert_into(cash_nonequip_items::table)
            .values(self.clone())
            .on_conflict(cash_nonequip_items::id)
            .do_update()
            .set(self.clone())
            .get_result::<Self>(&mut conn)
    }
}

impl EtcItemModel {
    pub async fn update_item(&self, state: &SharedState) -> QueryResult<Self> {
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
        diesel::insert_into(etc_items::table)
            .values(self.clone())
            .on_conflict(etc_items::id)
            .do_update()
            .set(self.clone())
            .get_result::<Self>(&mut conn)
    }
}

impl SetupItemModel {
    pub async fn update_item(&self, state: &SharedState) -> QueryResult<Self> {
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
        diesel::insert_into(setup_items::table)
            .values(self.clone())
            .on_conflict(setup_items::id)
            .do_update()
            .set(self.clone())
            .get_result::<Self>(&mut conn)
    }
}

impl UseItemModel {
    pub async fn update_item(&self, state: &SharedState) -> QueryResult<Self> {
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
        diesel::insert_into(use_items::table)
            .values(self.clone())
            .on_conflict(use_items::id)
            .do_update()
            .set(self.clone())
            .get_result::<Self>(&mut conn)
    }
}
