/* create_char/error.rs
 * The purpose of this module is to provide errors related to creating characters.
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

use assembly::item::error::ItemAssemblyError;
use assembly::job::error::JobAssemblyError;
use assembly::keybinding::error::KeybindingAssemblyError;
use assembly::skill::error::SkillAssemblyError;
use db::error::DatabaseError;
use domain::error::DomainError;
use entity::account::error::AccountEntityError;
use entity::character::error::CharacterEntityError;
use entity::item::error::ItemEntityError;
use entity::keybinding::error::KeybindingEntityError;
use entity::map::error::MapEntityError;
use entity::portal::error::PortalEntityError;
use entity::skill::error::SkillEntityError;
use metadata::item::error::ItemMetadataError;
use metadata::map::error::MapMetadataError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use state::error::StateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateCharError {
    #[error("Packet io error in create character layer")]
    IOError(#[from] IOError),

    #[error("Database error in create character layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Session error in create character layer")]
    SessionError(#[from] SessionError),

    #[error("Account entity error in create character layer")]
    AccountEntityError(#[from] AccountEntityError),

    #[error("Character entity error in create character layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("Map entity error in create character layer")]
    MapEntityError(#[from] MapEntityError),

    #[error("Skill entity error in create character layer")]
    SkillError(#[from] SkillEntityError),

    #[error("Item entity error in create character layer")]
    ItemEntityError(#[from] ItemEntityError),

    #[error("Keybinding entity error in create character layer")]
    KeybindingError(#[from] KeybindingEntityError),

    #[error("State error in create character layer")]
    StateError(#[from] StateError),

    #[error("Portal entity error in create character layer")]
    PortalEntityError(#[from] PortalEntityError),

    #[error("Domain error in create character layer")]
    DomainError(#[from] DomainError),

    #[error("Item assembly error in create character layer")]
    ItemAssemblyError(#[from] ItemAssemblyError),

    #[error("Job assembly error in create character layer")]
    JobAssemblyError(#[from] JobAssemblyError),

    #[error("Skill assembly error in create character layer")]
    SkillAssemblyError(#[from] SkillAssemblyError),

    #[error("Keybinding assembly error in create character layer")]
    KeybindingAssemblyError(#[from] KeybindingAssemblyError),

    #[error("Item metadata error in create character layer")]
    ItemMetadataError(#[from] ItemMetadataError),

    #[error("Map metadata error in create character layer")]
    MapMetadataError(#[from] MapMetadataError),

    #[error("Packet build error in create character layer")]
    PacketBuildError(#[from] PacketBuildError),
}
