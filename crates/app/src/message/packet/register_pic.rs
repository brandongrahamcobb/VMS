/* register_pic/message.rs
 * The purpose of this module is to handle PIC registration.
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
use ipc::data::register_pic::RegisterPicCommand;

#[derive(Message)]
pub struct RegisterPicMessage {
    pub client_id: i32,
    pub char_id: i32,
    pub mac: String,
    pub hwid: String,
    pub pic: String,
}

impl From<(RegisterPicMessage, i32, i16, u8)> for RegisterPicCommand {
    fn from((msg, acc_id, world_id, channel_id): (RegisterPicMessage, i32, i16, u8)) -> Self {
        Self {
            client_id: msg.client_id,
            acc_id,
            world_id,
            channel_id,
            char_id: msg.char_id,
            mac: msg.mac,
            hwid: msg.hwid,
            pic: msg.pic,
        }
    }
}
