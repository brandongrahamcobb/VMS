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
use crate::models::mob::error::MobError;
use crate::models::portal::error::PortalError;

#[derive(Debug, Error)]
pub enum MapError {
    #[error("Requested map was not found in map model layer. Map ID: {0}")]
    NotFound(i32),

    #[error("Metadata error map model layer")]
    MetadataError(#[from] MetadataError),

    #[error("Mob model error map model layer")]
    MobError(#[from] MobError),

    #[error("Portal model error map model layer")]
    PortalError(#[from] PortalError),
}
