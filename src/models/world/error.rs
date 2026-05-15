/* world/error.rs
 * The purpose of this module is to provide errors related to worlds.
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
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Requested world was not found in world model layer: {0}")]
    NotFound(i8),

    #[error("Config error in world model layer")]
    ConfigError(#[from] ConfigError),

    #[error("No worlds error in world model layer")]
    NoWorlds,

    #[error("No event message in world model layer: {0}")]
    NoEventMessage(i8),

    #[error("No name in world model layer: {0}")]
    NoName(i8),

    #[error("No flag in world model layer: {0}")]
    NoFlag(i8),

    #[error("World count exceeds the avaiable worlds in model layer")]
    CountExceedsAvailable,
}
