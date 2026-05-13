/* equip_stats/model.rs
 * The purpose of this module is to provide an equip statistics model and its implementation.
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

use crate::db::schema::equip_stats;
use crate::models::error::ModelError;
use crate::models::item::equip_stats::wrapper::EquipStats;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Identifiable, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = equip_stats)]
pub struct EquipStatsModel {
    pub id: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub luck: i32,
    pub attack: i32,
    pub weapon_defense: i32,
    pub magic: i32,
    pub magic_defense: i32,
    pub hp: i32,
    pub mp: i32,
    pub accuracy: i32,
    pub avoid: i32,
    pub hands: i32,
    pub speed: i32,
    pub jump: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl EquipStatsModel {
    pub fn load(&self) -> Result<EquipStats, ModelError> {
        Ok(EquipStats {
            model: self.clone(),
        })
    }
}
