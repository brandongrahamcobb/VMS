/* map/error.rs
 * The purpose of this module is to provide errors related to maps.
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

use crate::metadata::error::MetadataError;

#[derive(Debug, Error)]
pub enum MobError {
    #[error("Metadata error mob model layer")]
    MetadataError(#[from] MetadataError),

    #[error("No info value error in life metadata in mob model layer")]
    NoInfo,

    #[error("No type value error in life metadata in mob model layer")]
    NoType,

    #[error("Life is not a mob error in mob model layer")]
    NotMob,

    #[error("Not life value error in map metadata in mob model layer. Map WZ: {0}")]
    NoLife(i32),
}
