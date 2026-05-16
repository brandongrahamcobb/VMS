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

use crate::db::error::DatabaseError;
use crate::models::account::error::AccountError;
use crate::models::character::error::CharacterError;
use crate::models::item::error::ItemError;
use crate::models::keybinding::error::KeybindingError;
use crate::models::map::error::MapError;
use crate::models::skill::error::SkillError;
use crate::net::packet::codec::spawn_player::error::CodecSpawnPlayerError;
use crate::net::packet::io::error::IOError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateCharError {
    #[error("Packet io error in create character layer")]
    IOError(#[from] IOError),

    #[error("Database error in create character layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Session error in create character layer")]
    SessionError(#[from] SessionError),

    #[error("Account model error in create character layer")]
    AccountError(#[from] AccountError),

    #[error("Character model error in create character layer")]
    CharacterError(#[from] CharacterError),

    #[error("Codec spawn player error in create characters layer")]
    CodecSpawnPlayerError(#[from] CodecSpawnPlayerError),

    #[error("Map model error in create character layer")]
    MapError(#[from] MapError),

    #[error("Skill model error in create character layer")]
    SkillError(#[from] SkillError),

    #[error("Item model error in create character layer")]
    ItemError(#[from] ItemError),

    #[error("Keybinding model error in create character layer")]
    KeybindingError(#[from] KeybindingError),
}
