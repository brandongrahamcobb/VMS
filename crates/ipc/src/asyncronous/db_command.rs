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

use base::character::StatsUpdate;
use db::{character::model::CharacterModel, keybinding::model::KeybindingModel};

#[derive(Clone)]
pub enum DatabaseCommand {
    // Login
    LoginRequest {
        client_id: i32,
        username: String,
        password: String,
    },
    ListChars {
        client_id: i32,
        acc_id: i32,
        channel_id: u8,
        world_id: i16,
    },
    SetPic {
        client_id: i32,
        acc_id: i32,
        pic: String,
    },
    SetTosAccepted {
        client_id: i32,
        acc_id: i32,
    },
    CheckCharName {
        client_id: i32,
        ign: String,
    },
    SelectCharWithPic {
        client_id: i32,
        acc_id: i32,
        char_id: i32,
        mac: String,
        hwid: String,
        pic: String,
    },
    CreateCharRequest {
        client_id: i32,
        char_model: CharacterModel,
    },

    // In-Game
    UpdateKeybindings {
        client_id: i32,
        binds: Vec<KeybindingModel>,
    },
    UpdateStats {
        client_id: i32,
        char_id: i32,
        updates: Vec<StatsUpdate>,
    },
}
