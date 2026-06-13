/* app/src/message/packet/create_char.rs
 * The purpose of this module is to serve new character creation packet system messages.
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
use db::character::model::CharacterModel;
use db::item::model::ItemModel;
use db::keybinding::model::KeybindingModel;
use db::skill::model::SkillModel;
use std::collections::HashMap;

#[derive(Message)]
pub struct ReadCreateCharRequestMessage {
    pub client_id: i32,
    pub ign: String,
    pub job_wz: i16,
    pub face_wz: i32,
    pub hair_wz: i32,
    pub hair_color_wz: i32,
    pub skin_wz: i32,
    pub top_wz: i32,
    pub bottom_wz: i32,
    pub shoes_wz: i32,
    pub weapon_wz: i32,
    pub gender_wz: i16,
}

#[derive(Message)]
pub struct CreateCharResponseMessage {
    pub client_id: i32,
    pub char_id: i32,
    pub char_model: CharacterModel,
    pub equipped_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub equip_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub use_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub etc_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub setup_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub cash_item_model_map: HashMap<i32, Vec<ItemModel>>,
    pub keybinding_model_map: HashMap<i32, Vec<KeybindingModel>>,
    pub skill_model_map: HashMap<i32, Vec<SkillModel>>,
    pub equip_tab_inv_capacity_map: HashMap<i32, i16>,
    pub use_tab_inv_capacity_map: HashMap<i32, i16>,
    pub etc_tab_inv_capacity_map: HashMap<i32, i16>,
    pub setup_tab_inv_capacity_map: HashMap<i32, i16>,
    pub cash_tab_inv_capacity_map: HashMap<i32, i16>,
}
