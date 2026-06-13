/* app/src/message/packet/list_chars.rs
 * The purpose of this module is to serve character list packet system messages.
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

use bevy::prelude::Message;
use db::character::model::CharacterModel;
use db::item::model::ItemModel;
use db::keybinding::model::KeybindingModel;
use db::skill::model::SkillModel;

#[derive(Message)]
pub struct ReadListCharsRequestMessage {
    pub client_id: i32,
    pub channel_id: u8,
    pub world_id: i16,
}

#[derive(Message)]
pub struct ListCharsSuccessResponseMessage {
    pub client_id: i32,
    pub channel_id: u8,
    pub char_models: Vec<CharacterModel>,
    pub keybinding_model_map: HashMap<i32, Vec<KeybindingModel>>,
    pub skill_model_map: HashMap<i32, Vec<SkillModel>>,
    pub equipped_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub equip_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub use_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub etc_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub setup_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub cash_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub equip_tab_inv_capacity_map: HashMap<i32, i16>,
    pub use_tab_inv_capacity_map: HashMap<i32, i16>,
    pub etc_tab_inv_capacity_map: HashMap<i32, i16>,
    pub setup_tab_inv_capacity_map: HashMap<i32, i16>,
    pub cash_tab_inv_capacity_map: HashMap<i32, i16>,
    pub slots: i16,
    pub world_id: i16,
}

#[derive(Message)]
pub struct ListCharsFailedResponseMessage {
    pub client_id: i32,
}
