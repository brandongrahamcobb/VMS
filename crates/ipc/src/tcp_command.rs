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

use entity::keybinding::model::KeybindingModel;
use net::packet::model::Packet;

pub enum TcpCommand {
    // General
    SendPacket {
        client_id: i32,
        packet: Packet,
    },
    Disconnect {
        client_id: i32,
    },
    AcceptTransition {
        client_id: i32,
        port: i16,
    },

    // Login
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
    // CredentialsCommand{
    //     client-id: i32,
    //
    //
    //
    //     ,
    SelectCharWithPic {
        client_id: i32,
        acc_id: i32,
        char_id: i32,
        mac: String,
        hwid: String,
        pic: String,
    },
    //
    // // Character
    // ListChars(ListCharsCommand),
    // CreateChar(CreateCharCommand),
    // DeleteChar(DeleteCharCommand),
    // CheckCharacterName(CheckCharNameCommand),
    //
    // In-Game
    UpdateKeybindings {
        client_id: i32,
        binds: Vec<KeybindingModel>,
    },
    UpdateHealth {
        client_id: i32,
        char_id: i32,
        hp: i16,
    },
    // PlayerLoggedIn(PlayerLoggedInCommand),
    //
    //
    // ChangeKeymap(ChangeKeymapCommand),
    // ChangeSkill {
    //     client_id: i32,
    //     char_id: i32,
    //     skill: Skill,
    // },
    // PickupItem(PickupItemCommand),
    // RandomizeDrop {
    //     client_id: i32,
    //     mob_id: i32,
    // },
}
