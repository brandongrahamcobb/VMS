/* list_chars/message.rs
 * The purpose of this module is to handle character listing.
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

use core::convert::From;

use bevy::prelude::Message;
use entity::character::wrapper::Character;
use ipc::data::list_chars::ListCharsCommand;

#[derive(Message)]
pub struct ListCharsRequestMessage {
    pub client_id: i32,
    pub channel_id: u8,
    pub world_id: i16,
}

impl From<(ListCharsRequestMessage, i32)> for ListCharsCommand {
    fn from((msg, acc_id): (ListCharsRequestMessage, i32)) -> Self {
        Self {
            client_id: msg.client_id,
            acc_id,
            channel_id: msg.channel_id,
            world_id: msg.world_id,
        }
    }
}

#[derive(Message)]
pub struct ListCharsSuccessMessage {
    pub client_id: i32,
    pub channel_id: u8,
    pub chars: Vec<Character>,
    pub slots: i16,
    pub world_id: i16,
}

#[derive(Message)]
pub struct ListCharsFailedMessage {
    pub client_id: i32,
}
