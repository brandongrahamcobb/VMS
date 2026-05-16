/* character/service.rs
 * The purpose of this module is to provide assisting functions and implementations for characters.
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
use crate::models::character::error::CharacterError;
use crate::models::character::wrapper::Character;
use crate::models::{character, item, keybinding, skill};
use crate::runtime::state::SharedState;

pub async fn get_char_by_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Character, CharacterError> {
    let char_model = character::query::getters::get_char_model_by_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    let char = char_model.load(state).await?;
    Ok(char)
}

pub async fn delete_character_by_id(
    state: &SharedState,
    char_id: i32,
) -> Result<(), CharacterError> {
    character::query::setters::delete_char_by_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    item::query::setters::delete_items_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    keybinding::query::setters::delete_keybindings_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    skill::query::setters::delete_skills_by_char_id(state, char_id)
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    // delete associated skills shard between chars
    Ok(())
}
