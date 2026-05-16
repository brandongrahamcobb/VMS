/* keybinding/service.rs
 * The purpose of this module is to provide assisting functions for keybindings.
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

use std::collections::HashMap;

use crate::db::error::DatabaseError;
use crate::models::keybinding;
use crate::models::keybinding::error::KeybindingError;
use crate::models::keybinding::wrapper::Keybinding;
use crate::runtime::state::SharedState;

pub async fn load_keybindings(
    state: &SharedState,
    char_id: i32,
) -> Result<HashMap<i32, Keybinding>, KeybindingError> {
    let keybinding_models =
        keybinding::query::getters::get_keybinding_models_by_char_id(state, char_id)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
    Ok(keybinding_models
        .into_iter()
        .map(|k| -> Result<(i32, Keybinding), KeybindingError> { Ok((k.key, k.load()?)) })
        .collect::<Result<HashMap<i32, Keybinding>, KeybindingError>>()?)
}
