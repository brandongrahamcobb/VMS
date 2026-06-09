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
use db::error::DatabaseError;
use metadata::item::error::ItemMetadataError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AsyncDomainError {
    #[error("Configuration error in domain layer")]
    ConfigError(#[from] ConfigError),

    #[error("Database error in domain layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Item error in domain layer")]
    ItemError,

    #[error("Item metadata error in domain layer")]
    ItemMetadataError(#[from] ItemMetadataError),

    #[error("Bcrypt error in domain layer")]
    CryptError(#[from] BcryptError),
}
