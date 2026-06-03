/* player_logged_in/message.rs
 * The purpose of this module is to handle player login.
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
use bevy::prelude::Message;
use db::{item::model::ItemModel, keybinding::model::KeybindingModel, skill::model::SkillModel};

#[derive(Message)]
pub struct ReadPlayerLoggedInRequestMessage {
    pub client_id: i32,
    pub char_id: i32,
    pub channel_id: i16,
}

#[derive(Message)]
pub struct PlayerLoggedInResponseMessage {
    pub client_id: i32,
    pub keybinding_models: Vec<KeybindingModel>,
    pub skill_models: Vec<SkillModel>,
    pub equip_tab_capacity: i16,
    pub use_tab_capacity: i16,
    pub etc_tab_capacity: i16,
    pub setup_tab_capacity: i16,
    pub cash_tab_capacity: i16,
    pub equipped_item_models: Vec<ItemModel>,
    pub equip_tab_item_models: Vec<ItemModel>,
    pub use_tab_item_models: Vec<ItemModel>,
    pub etc_tab_item_models: Vec<ItemModel>,
    pub setup_tab_item_models: Vec<ItemModel>,
    pub cash_tab_item_models: Vec<ItemModel>,
}
