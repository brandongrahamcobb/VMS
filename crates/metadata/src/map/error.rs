/* metadata/src/map/error.rs
 * The purpose of this module is to provide errors related to metadata of jobs.
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

use core::num::ParseIntError;

use thiserror::Error;

use crate::error::MetadataServiceError;

#[derive(Debug, Error)]
pub enum MapMetadataError {
    #[error("Portal error in map metadata layer")]
    PortalError,

    #[error("Mob error in map metadata layer")]
    MobError,

    #[error("Death map error in map metadata layer")]
    DeathMapError,

    #[error("Parse integer error in map metadata layer")]
    ParseIntError(#[from] ParseIntError),

    #[error("Metadata service error in map metadata layer")]
    MetadataServiceError(#[from] MetadataServiceError),
}
