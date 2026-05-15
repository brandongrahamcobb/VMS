/* models/error.rs
 * The purpose of this module is to provide errors related to objects/structs.
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
use crate::metadata::error::MetadataError;
use crate::models::account::error::AccountError;
use crate::models::channel::error::ChannelError;
use crate::models::character::error::CharacterError;
use crate::models::item::error::ItemError;
use crate::models::keybinding::error::KeybindingError;
use crate::models::map::error::MapError;
use crate::models::portal::error::PortalError;
use crate::models::world::error::WorldError;
use diesel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Account error in model layer")]
    AccountError(#[from] AccountError),

    #[error("Character error in model layer")]
    CharacterError(#[from] CharacterError),

    #[error("Channel error in model layer")]
    ChannelError(#[from] ChannelError),

    #[error("Keybinding error in model layer")]
    KeybindingError(#[from] KeybindingError),

    #[error("World error in model layer")]
    WorldError(#[from] WorldError),

    #[error("Config error in model layer")]
    ConfigError(#[from] ConfigError),

    #[error("Metadata error in model layer")]
    MetadataError(#[from] MetadataError),

    #[error("Diesel error in model layer")]
    DieselError(#[from] diesel::result::Error),

    #[error("Map error in model layer")]
    MapError(#[from] MapError),

    #[error("Item error in model layer")]
    ItemError(#[from] ItemError),

    #[error("Portal error in model layer")]
    PortalError(#[from] PortalError),
}
