/* drop/model.rs
 * The purpose of this module is to provide a drop item model.
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

use crate::db::schema::drop_items;
use crate::models::error::ModelError;
use crate::models::item::drop::wrapper::DropItem;
use crate::models::item::equip_stats;
use crate::models::item::equip_stats::model::EquipStatsModel;
use crate::runtime::state::SharedState;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = drop_items)]
pub struct DropItemModel {
    pub id: i32,
    pub wz: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl DropItemModel {
    pub async fn load(&self, state: &SharedState) -> Result<DropItem, ModelError> {
        let equip_stats_model: EquipStatsModel =
            equip_stats::query::getters::get_equip_stats_model_by_id(state, self.id).await?;
        Ok(DropItem {
            equip_stats: equip_stats_model.load()?.clone(),
            model: self.clone(),
        })
    }
}
