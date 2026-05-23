/* assembly/src/character/error.rs
 * The purpose of this module is to provide errors related to assembling characters.
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
use crate::item::error::ItemAssemblyError;
use crate::keybinding::error::KeybindingAssemblyError;
use crate::skill::error::SkillAssemblyError;
use db::error::DatabaseError;
use entity::character::error::CharacterEntityError;
use entity::item::error::ItemEntityError;
use metadata::job::error::JobMetadataError;
use metadata::map::error::MapMetadataError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CharacterAssemblyError {
    #[error("Database error in character assembly layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Item assembly error in character assembly layer")]
    ItemAssemblyError(#[from] ItemAssemblyError),

    #[error("Item entity error in character assembly layer")]
    ItemEntityError(#[from] ItemEntityError),

    #[error("Character entity error in character assembly layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("Skill assembly error in character assembly layer")]
    SkillAssemblyError(#[from] SkillAssemblyError),

    #[error("Skill assembly error in character assembly layer")]
    KeybindingAssemblyError(#[from] KeybindingAssemblyError),

    #[error("Map metadata error in character assembly layer")]
    MapMetadataError(#[from] MapMetadataError),

    #[error("Job metadata error in character assembly layer")]
    JobMetadataError(#[from] JobMetadataError),
}
