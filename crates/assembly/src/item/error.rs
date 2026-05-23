/* assembly/src/item/error.rs
 * The purpose of this module is to provide errors related to assembling items or inventories.
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

use db::error::DatabaseError;
use entity::item::error::ItemEntityError;
use metadata::item::error::ItemMetadataError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ItemAssemblyError {
    #[error("Item metadata error in item assembly layer")]
    ItemMetadataError(#[from] ItemMetadataError),

    #[error("Item entity error in item assembly layer")]
    ItemEntityError(#[from] ItemEntityError),

    #[error("Database error in item assembly layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Inventory error in item assembly layer")]
    InventoryError,
}
