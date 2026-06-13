/* base/src/keybinding.rs
 * The purpose of this module is to provide base keybinding values.
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

pub const DEFAULT_KEY: [i32; 23] = [
    59, 60, 61, 62, 63, 64, 65, 56, 87, 18, 23, 31, 37, 19, 17, 46, 50, 16, 43, 40, 21, 4, 84,
];

pub const DEFAULT_TYPE: [i16; 23] = [
    6, 6, 6, 6, 6, 6, 6, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];

pub const DEFAULT_ACTION: [i32; 23] = [
    100, 101, 102, 103, 104, 105, 106, 54, 54, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15,
];
