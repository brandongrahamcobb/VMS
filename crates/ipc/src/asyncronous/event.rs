/* ipc/src/event.rs
 * The purpose of this module is to provide an enum for TCP events.
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

use base::account::{FailedCode, StatusCode};
use db::{account::model::AccountModel, character::model::CharacterModel};
use net::packet::model::Packet;

pub enum AsyncEvent {
    // General
    PacketReceived {
        client_id: i32,
        packet: Packet,
    },
    ClientTransitioning {
        client_id: i32,
        channel_id: u8,
        world_id: i16,
    },
    ClientConnected {
        client_id: i32,
    },
    ClientDisconnected {
        client_id: i32,
    },

    // Login
    LoginSuccess {
        client_id: i32,
        acc_model: AccountModel,
        status: StatusCode,
    },
    LoginFailed {
        client_id: i32,
        code: FailedCode,
    },
    CharCreated {
        client_id: i32,
        char_model: CharacterModel,
    },
    ListCharsSuccess {
        client_id: i32,
        channel_id: u8,
        char_models: Vec<CharacterModel>,
        slots: i16,
        world_id: i16,
    },
    ListCharsFailed {
        client_id: i32,
    },
    CheckCharName {
        client_id: i32,
        exists: bool,
        ign: String,
    },
    SelectCharWithPic {
        client_id: i32,
        char_id: i32,
        status: bool,
    },
}
