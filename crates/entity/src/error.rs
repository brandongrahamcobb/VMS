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

use crate::account::error::AccountEntityError;
use crate::channel::error::ChannelEntityError;
use crate::character::error::CharacterEntityError;
use crate::item::error::ItemEntityError;
use crate::map::error::MapEntityError;
use crate::portal::error::PortalEntityError;
use crate::world::error::WorldEntityError;
use config::error::ConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Account error in entity layer")]
    AccountEntityError(#[from] AccountEntityError),

    #[error("Character error in entity layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("Channel error in entity layer")]
    ChannelEntityError(#[from] ChannelEntityError),

    #[error("World error in entity layer")]
    WorldEntityError(#[from] WorldEntityError),

    #[error("Config error in entity layer")]
    ConfigError(#[from] ConfigError),

    #[error("Map error in entity layer")]
    MapEntityError(#[from] MapEntityError),

    #[error("Item error in entity layer")]
    ItemEntityError(#[from] ItemEntityError),

    #[error("Portal error in entity layer")]
    PortalEntityError(#[from] PortalEntityError),
}
