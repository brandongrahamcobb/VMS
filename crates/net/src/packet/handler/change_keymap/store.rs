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

use crate::packet::handler::change_keymap::error::ChangeKeymapError;
use crate::packet::handler::change_keymap::reader::ChangeKeymapReader;
use db;
use db::pool::DbPool;
use entity::keybinding::model::KeybindingModel;
use itertools::izip;
use session::model::Session;
use std::time::SystemTime;

pub struct ChangeKeymapStore;

impl ChangeKeymapStore {
    pub async fn store_change_keymap(
        pool: &DbPool,
        session: &Session,
        reader: &ChangeKeymapReader,
    ) -> Result<Self, ChangeKeymapError> {
        let char_id = session.get_char_id()?;
        let new_binds: Vec<KeybindingModel> = izip!(
            reader.keys.clone(),
            reader.types.clone(),
            reader.model.clone()
        )
        .map(
            |(key, bind_type, action): (i32, i16, i32)| KeybindingModel {
                id: None,
                char_id,
                key,
                bind_type,
                action,
                created_at: None,
                updated_at: SystemTime::now(),
            },
        )
        .collect();
        db::keybinding::setters::update_keybindings(pool, new_binds.clone()).await?;
        return Ok(Self);
    }
}
