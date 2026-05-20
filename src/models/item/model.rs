/* item/model.rs
 * The purpose of this module is to provide an item model.
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
use crate::db::schema::drops;
use crate::models::item::error::ItemError;
use diesel::{Identifiable, Queryable};

pub trait ItemModel {
    fn id(&self) -> Option<i32>;
    fn ipos(&self) -> Option<i16>;

    fn get_id(&self) -> Result<i32, ItemError> {
        self.id().ok_or(ItemError::NoId)
    }

    fn get_ipos(&self) -> Result<i16, ItemError> {
        self.ipos().ok_or(ItemError::NoPos)
    }
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = drops)]
pub struct DropData {
    pub id: i64,
    pub mob_wz: i32,
    pub item_wz: i32,
    pub minimum_quantity: i32,
    pub maximum_quantity: i32,
    pub chance: i32,
}
