/* metadata/src/item/error.rs
 * The purpose of this module is to provide errors related to metadata of items.
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

use thiserror::Error;

use crate::error::MetadataServiceError;

#[derive(Debug, Error)]
pub enum ItemMetadataError {
    #[error("Metadata service error in item metadata layer")]
    MetadataServiceError(#[from] MetadataServiceError),

    #[error("Equip error in item metadata layer")]
    EquipError,

    #[error("Non-equip error in item metadata layer")]
    NonequipError,

    #[error("Inventory error in item metadata layer")]
    InventoryError,
}
