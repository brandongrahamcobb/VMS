/* assembly/src/item/disassemble.rs
 * The purpose of this module is to disassemble an item wrapper.
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
use crate::item::error::ItemAssemblyError;
use db;
use db::pool::DbPool;
use entity::item::wrapper::Item;

async fn disassemble_item(pool: &DbPool, item: Item) -> Result<(), ItemAssemblyError> {
    db::item::setters::delete_item_by_id(pool, item.model.get_id()?).await?;
    Ok(())
}
