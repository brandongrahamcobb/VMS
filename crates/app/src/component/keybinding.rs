/* app/src/component/keybinding.rs
 * The purpose of this module is to provide a keybinding component.
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

use bevy::ecs::component::Component;
use core::convert::From;
use db::keybinding::model::KeybindingModel;

#[derive(Component)]
pub struct MapleKeybinding {
    pub id: i32,
    pub char_id: i32,
    pub key: i32,
    pub bind_type: i16,
    pub action: i32,
}

impl From<KeybindingModel> for MapleKeybinding {
    fn from(model: KeybindingModel) -> Self {
        let id = if let Some(id) = model.id { id } else { 0 };
        Self {
            id,
            char_id: model.char_id,
            key: model.key,
            bind_type: model.bind_type,
            action: model.action,
        }
    }
}
