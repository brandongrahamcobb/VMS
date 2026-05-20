/* character/error.rs
 * The purpose of this module is to provide errors related to characters.
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

use crate::db::error::DatabaseError;
use crate::models::item::error::ItemError;
use crate::models::job::error::JobError;
use crate::models::keybinding::error::KeybindingError;
use crate::models::portal::error::PortalError;
use crate::models::skill::error::SkillError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CharacterError {
    #[error("No id found in character model layer")]
    NoId,

    #[error("Item model error in character model layer")]
    ItemError(#[from] ItemError),

    #[error("Keybinding model error in character model layer")]
    KeybindingError(#[from] KeybindingError),

    #[error("Skill model error in character model layer")]
    SkillError(#[from] SkillError),

    #[error("Portal model error in character model layer")]
    PortalError(#[from] PortalError),

    #[error("Job model error in character model layer")]
    JobError(#[from] JobError),

    #[error("Database error in character model layer")]
    DatabaseError(#[from] DatabaseError),
}
