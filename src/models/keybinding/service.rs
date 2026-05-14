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

use crate::models::keybinding;
use crate::models::keybinding::model::KeybindingModel;
use crate::models::keybinding::wrapper::Keybinding;
use crate::models::error::ModelError;
use crate::runtime::state::SharedState;

pub async fn get_keybindings_by_char_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Keybinding>, ModelError> {
    let mut keybindings: Vec<Keybinding> = Vec::<Keybinding>::new();
    let keybinding_models: Vec<KeybindingModel> =
        keybinding::query::getters::get_keybinding_models_by_character_id(state, char_id).await?;
    for keybinding_model in keybinding_models {
        keybindings.push(keybinding_model.load()?);
    }
    Ok(keybindings)
}
