/* db/src/inventory/getters.rs
 * The purpose of this module is to provide database getters for inventories.
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
use crate::inventory::model::InventoryCapacityModel;
use crate::pool::{self, DbPool};
use crate::schema::inventory_capacity;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn get_inventory_slot_capacities_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<InventoryCapacityModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        inventory_capacity::table
            .filter(inventory_capacity::char_id.eq(char_id))
            .get_result::<InventoryCapacityModel>(conn)
    })
    .await
}

pub async fn get_equip_slot_capacity_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<i16, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        inventory_capacity::table
            .filter(inventory_capacity::char_id.eq(char_id))
            .select(inventory_capacity::equip_slot_capacity)
            .get_result::<i16>(conn)
    })
    .await
}

pub async fn get_use_slot_capacity_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<i16, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        inventory_capacity::table
            .filter(inventory_capacity::char_id.eq(char_id))
            .select(inventory_capacity::use_slot_capacity)
            .get_result::<i16>(conn)
    })
    .await
}

pub async fn get_etc_slot_capacity_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<i16, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        inventory_capacity::table
            .filter(inventory_capacity::char_id.eq(char_id))
            .select(inventory_capacity::etc_slot_capacity)
            .get_result::<i16>(conn)
    })
    .await
}

pub async fn get_setup_slot_capacity_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<i16, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        inventory_capacity::table
            .filter(inventory_capacity::char_id.eq(char_id))
            .select(inventory_capacity::setup_slot_capacity)
            .get_result::<i16>(conn)
    })
    .await
}

pub async fn get_cash_slot_capacity_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<i16, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        inventory_capacity::table
            .filter(inventory_capacity::char_id.eq(char_id))
            .select(inventory_capacity::cash_slot_capacity)
            .get_result::<i16>(conn)
    })
    .await
}
