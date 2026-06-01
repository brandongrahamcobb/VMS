/* db/error.rs
 * The purpose of this module is to provide errors related to the database.
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

use bcrypt::BcryptError;
use db::item::error::ItemModelError;
use metadata::{item::error::ItemMetadataError, job::error::JobMetadataError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyncDomainError {
    #[error("Item model error in syncronous domain layer")]
    ItemModelError(#[from] ItemModelError),

    #[error("Job metadata error in syncronous domain layer")]
    JobMetadataError(#[from] JobMetadataError),

    #[error("Bcrypt error in syncronous domain layer")]
    BcryptError(#[from] BcryptError),

    #[error("Inventory full in syncronous domain layer")]
    InventoryFull,

    #[error("Item error in syncronous domain layer")]
    ItemError,

    #[error("Item metadata error in syncronous domain layer")]
    ItemMetadataError(#[from] ItemMetadataError),
}
