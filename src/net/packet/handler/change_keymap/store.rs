/* change_keymap/store.rs
 * The purpose of this module is to resolve relevant variables for changing keymaps.
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

use std::time::SystemTime;

use crate::models::keybinding;
use crate::models::keybinding::model::KeybindingModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_keymap::reader::ChangeKeymapReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

#[derive(Clone)]
pub struct ChangeKeymapStore;

impl ChangeKeymapStore {
    pub async fn store_change_keymap(
        state: &SharedState,
        session: Session,
        reader: ChangeKeymapReader,
    ) -> Result<Self, NetworkError> {
        let char = session.get_active_char(state).await?;
        let char_id = char.model.get_id()?;
        let new_binds: Vec<KeybindingModel> = izip!(
            reader.keys.clone(),
            reader.types.clone(),
            reader.model.clone()
        )
        .map(
            |(key, bind_type, action): (i32, i16, i32)| KeybindingModel {
                char_id,
                key,
                bind_type,
                action,
                created_at: None,
                updated_at: SystemTime::now(),
            },
        )
        .collect();
        keybinding::query::setters::update_keybindings(state, new_binds.clone()).await?;
        return Ok(Self);
    }
}
