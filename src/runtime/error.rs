/* runtime/error.rs
 * The purpose of this module is to provide errors related to the runtime loop.
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
use crate::db::error::DatabaseError;
use crate::models::world::error::WorldError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Config error in runtime layer")]
    ConfigError(#[from] ConfigError),

    #[error("Concurrency join error in runtime layer")]
    JoinError(#[from] JoinError),

    #[error("Unexpected end of output in runtime layer")]
    UnexpectedOf(#[from] std::io::Error),

    #[error("Environment loading error in runtime layer")]
    DotenvError(#[from] dotenvy::Error),

    #[error("Failed database in runtime layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("State error in runtime layer")]
    StateError(#[from] StateError),
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Map not found in game state layer: {0}")]
    NoMap(i32),

    #[error("Mob not found in game state layer: {0}")]
    NoMob(u32),

    #[error("Channel not found in game state layer: {0}")]
    NoChannel(u8),

    #[error("World not found in game state layer: {0}")]
    NoWorld(i16),

    #[error("Config error in game state layer")]
    ConfigError(#[from] ConfigError),

    #[error("Failed database in game state layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("World model error in game state layer")]
    WorldError(#[from] WorldError),
}
