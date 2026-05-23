/* metadata/src/error.rs
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

use config::error::ConfigError;
use core::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum MetadataServiceError {
    #[error("Requested metadata resource was not found in metadata entity layer: {0}")]
    NotFound(i32),

    #[error("Metadata anyhow error in metadata entity layer")]
    AnyHowError(#[from] anyhow::Error),

    #[error("Wz binrw error in metadata entity layer")]
    BinRwError(#[from] binrw::error::Error),

    #[error("Wz version error in metadata entity layer")]
    NoVersion,

    #[error("WzDirEntry error in metadata entity layer")]
    EntryError,

    #[error("Wz part error in metadata entity layer")]
    PartError,

    #[error("Wz Object error in metadata entity layer")]
    ObjectError,

    #[error("Config error in entity layer")]
    ConfigError(#[from] ConfigError),

    #[error("Parse integer error in metadata entity layer")]
    ParseIntError(#[from] ParseIntError),
}
