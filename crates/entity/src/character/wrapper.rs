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

use crate::character::model::CharacterModel;
use crate::item::wrapper::Inventory;
use crate::job::wrapper::Job;
use crate::keybinding::wrapper::Keybinding;
use crate::map::model::Point;
use crate::skill::wrapper::Skill;

pub struct Character {
    pub model: CharacterModel,
    pub binds: HashMap<i32, Keybinding>,
    pub job: Job,
    pub inventory: Inventory,
    pub skills: HashMap<i32, Skill>,
    pub pos: Point,
}
