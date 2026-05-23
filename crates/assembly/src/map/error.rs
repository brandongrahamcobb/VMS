/* assembly/src/map/error.rs
 * The purpose of this module is to provide errors related to map assembly.
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

use crate::mob::error::MobAssemblyError;
use crate::portal::error::PortalAssemblyError;
use metadata::map::error::MapMetadataError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MapAssemblyError {
    #[error("Map metadata error map assembly layer")]
    MapMetadataError(#[from] MapMetadataError),

    #[error("Mob assembly error map assembly layer")]
    MobAssemblyError(#[from] MobAssemblyError),

    #[error("Portal assembly error map assembly layer")]
    PortalAssemblyError(#[from] PortalAssemblyError),
}
