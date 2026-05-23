/* keybinding/model.rs
 * The purpose of this module is to provide a keybinding model.
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

use crate::keybinding::error::KeybindingEntityError;
use crate::schema::keybindings;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = keybindings)]
pub struct KeybindingModel {
    pub id: Option<i32>,
    pub char_id: i32,
    pub key: i32,
    pub bind_type: i16,
    pub action: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

#[derive(Debug, Clone, Copy)]
pub enum KeybindType {
    Nil = 0,
    Skill = 1,
    Item = 2,
    Cash = 3,
    Menu = 4,
    Action = 5,
    Face = 6,
    Macro = 7,
    Text = 8,
}

impl KeybindingModel {
    pub fn get_id(&self) -> Result<i32, KeybindingEntityError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(KeybindingEntityError::NoId)
        }
    }
}
