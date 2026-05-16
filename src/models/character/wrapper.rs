/* character/model.rs
 * The purpose of this module is to provide a character model and its wrapper.
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

use crate::models::character::model::CharacterModel;
use crate::models::item::wrapper::Inventory;
use crate::models::job::wrapper::Job;
use crate::models::keybinding::wrapper::Keybinding;
use crate::models::skill::wrapper::Skill;

pub struct Character {
    pub model: CharacterModel,
    pub binds: HashMap<i32, Keybinding>,
    pub job: Job,
    pub inventory: Inventory,
    pub skills: HashMap<i32, Skill>,
}

impl Character {
    pub fn update_model(&mut self, char_model: CharacterModel) {
        self.model = char_model.clone();
    }
}
