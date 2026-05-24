/* assembly/src/keybinding/assemble.rs
 * The purpose of this module is to assemble a keybinding wrapper.
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

use db;
use db::pool::DbPool;
use entity::keybinding::model::KeybindingModel;
use entity::keybinding::wrapper::Keybinding;
use std::collections::HashMap;

use crate::keybinding::error::KeybindingAssemblyError;

pub async fn assemble_keybindings_by_char_id(
    pool: &DbPool,
    char_id: i32,
) -> Result<HashMap<i32, Keybinding>, KeybindingAssemblyError> {
    let keybinding_models =
        db::keybinding::getters::get_keybinding_models_by_char_id(pool, char_id).await?;
    for keybinding_model in keybinding_models {
        binds.insert(
            keybinding_model.key,
            Keybinding {
                model: keybinding_model,
            },
        );
    }
    Ok(keybindings)
}

pub async fn assemble_keybinding_by_id(
    pool: &DbPool,
    bind_id: i32,
) -> Result<Keybinding, KeybindingAssemblyError> {
    let keybinding_model =
        db::keybinding::getters::get_keybinding_model_by_id(pool, bind_id).await?;
    let keybinding: Keybinding = Keybinding {
        model: keybinding_model,
    };
    Ok(keybinding)
}
