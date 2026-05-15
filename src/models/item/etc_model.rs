/* item/etc_model.rs
 * The purpose of this module is to provide an etc item model and its implementation.
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

use crate::db::schema::etc_items;
use crate::models::item::model::ItemModel;
use crate::models::item::wrapper::EtcItem;
use crate::runtime::state::SharedState;
use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = etc_items)]
pub struct EtcItemModel {
    pub id: Option<i32>,
    pub char_id: Option<i32>,
    pub wz: i32,
    pub ipos: Option<i16>,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl EtcItemModel {
    pub fn load(&self) -> EtcItem {
        EtcItem {
            model: self.clone(),
        }
    }

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

impl ItemModel for EtcItemModel {
    fn id(&self) -> Option<i32> {
        self.id
    }
    fn ipos(&self) -> Option<i16> {
        self.ipos
    }
}
