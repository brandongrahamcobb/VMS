/* wz/error.rs
 * The purpose of this module is to provide errors related to Wz metadata.
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

use crate::config::error::ConfigError;

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("Requested metadata resource was not found in metadata model layer: {0}")]
    NotFound(i32),

    #[error("Metadata anyhow error in metadata model layer")]
    AnyHowError(#[from] anyhow::Error),

    #[error("Wz binrw error in metadata model layer")]
    BinRwError(#[from] binrw::error::Error),

    #[error("Wz version error in metadata model layer")]
    NoVersion,

    #[error("WzDirEntry error in metadata model layer")]
    EntryError,

    #[error("Wz part error in metadata model layer")]
    PartError,

    #[error("Wz Object error in metadata model layer")]
    ObjectError,

    #[error("Config error in model layer")]
    ConfigError(#[from] ConfigError),
}
