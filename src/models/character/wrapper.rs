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

use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::model::CharacterModel;
use crate::models::character::skill::model::Skill;
use crate::models::item::inventory::wrapper::InventoryItem;

#[derive(Clone)]
pub struct Character {
    pub model: CharacterModel,
    pub binds: Vec<Keybinding>,
    pub items: Vec<InventoryItem>,
    pub skills: Vec<Skill>,
}

impl Character {
    pub fn update_model(&mut self, char_model: CharacterModel) {
        self.model = char_model.clone();
    }
}
