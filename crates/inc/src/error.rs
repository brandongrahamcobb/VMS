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
use config::error::ConfigError;
use db::{error::DatabaseError, item::error::ItemModelError};
use metadata::{item::error::ItemMetadataError, job::error::JobMetadataError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IncError {
    #[error("Item model error in inc layer")]
    ItemModelError(#[from] ItemModelError),

    #[error("Job metadata error in inc layer")]
    JobMetadataError(#[from] JobMetadataError),

    #[error("Bcrypt error in inc layer")]
    BcryptError(#[from] BcryptError),

    #[error("Inventory full in inc layer")]
    InventoryFull,

    #[error("Item error in inc layer")]
    ItemError,

    #[error("Item metadata error in inc layer")]
    ItemMetadataError(#[from] ItemMetadataError),

    #[error("Database error in inc layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Configuration error in inc layer")]
    ConfigError(#[from] ConfigError),
}
