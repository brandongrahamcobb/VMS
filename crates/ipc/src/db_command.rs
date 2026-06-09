/* ipc/src/command.rs
 * The purpose of this module is to provide an enum for TCP commands.
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

use base::character::StatsUpdate;
use db::{
    character::model::CharacterModel, item::model::ItemModel, keybinding::model::KeybindingModel,
    skill::model::SkillModel,
};

#[derive(Clone)]
pub enum DatabaseCommand {
    // Login
    LoginRequest {
        client_id: i32,
        username: String,
        password: String,
    },
    ListCharsRequest {
        client_id: i32,
        acc_id: i32,
        channel_id: u8,
        world_id: i16,
    },
    ChangePicRequest {
        client_id: i32,
        acc_id: i32,
        pic: String,
    },
    AcceptTosRequest {
        client_id: i32,
        acc_id: i32,
    },
    CharNameRequest {
        client_id: i32,
        ign: String,
    },
    SelectCharWithPicRequest {
        client_id: i32,
        acc_id: i32,
        char_id: i32,
        mac: String,
        hwid: String,
        pic: String,
    },
    JoinRequest {
        client_id: i32,
        char_id: i32,
    },
    CreateCharRequest {
        client_id: i32,
        char_model: CharacterModel,
        top_wz: i32,
        bottom_wz: i32,
        shoes_wz: i32,
        weapon_wz: i32,
    },
    FinishCharRequest {
        client_id: i32,
        equip_models: Vec<ItemModel>,
        keybinding_models: Vec<KeybindingModel>,
        skill_models: Vec<SkillModel>,
    },
    DeleteCharRequest {
        client_id: i32,
        char_id: i32,
    },
    UpdateMapRequest {
        client_id: i32,
        char_id: i32,
        map_wz: i32,
    },

    // In-Game
    PickupItem {
        client_id: i32,
        char_id: i32,
        item_id: i32,
        ipos: i16,
        pet_pickup: bool,
    },
    UpdateKeybindings {
        client_id: i32,
        binds: Vec<KeybindingModel>,
    },
    UpdateStats {
        client_id: i32,
        char_id: i32,
        updates: Vec<StatsUpdate>,
    },

    DeadMobRequest {
        client_id: i32,
        mob_id: u32,
    },
    CloseAttackRequest {
        client_id: i32,
        char_id: i32,
        count: i16,
        skill_id: i32,
        display: i16,
        toleft: i16,
        stance: i16,
        speed: i16,
        mob_damages: HashMap<u32, Vec<i32>>,
    },
}
