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
use crate::inventory::model::InventoryCapacityModel;
use crate::pool::{self, DbPool};
use crate::schema::inventory_capacity;
use diesel::RunQueryDsl;

pub async fn update_inventory_capacity(
    pool: &DbPool,
    inventory_capacity_model: &InventoryCapacityModel,
) -> Result<InventoryCapacityModel, DatabaseError> {
    let inventory_capacity_model: InventoryCapacityModel = inventory_capacity_model.clone();
    pool::spawn_db(pool, move |conn| {
        diesel::insert_into(inventory_capacity::table)
            .values(inventory_capacity_model.clone())
            .on_conflict((inventory_capacity::id, inventory_capacity::char_id))
            .do_update()
            .set(inventory_capacity_model.clone())
            .get_result::<InventoryCapacityModel>(conn)
    })
    .await
}
