/* select_char_with_PIC/message.rs
 * The purpose of this module is to handle PIC, character selection.
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
use ipc::data::select_char_with_pic::SelectCharWithPic;

#[derive(Message)]
pub struct SelectCharWithPicRequestMessage {
    pub client_id: i32,
    pub char_id: i32,
    pub mac: String,
    pub hwid: String,
    pub pic: String,
}

impl From<(SelectCharWithPicMessage, i32)> for SelectCharWithPic {
    fn from((msg, acc_id): (SelectCharWithPicMessage, i32)) -> Self {
        Self {
            client_id: msg.client_id,
            acc_id,
            char_id: msg.char_id,
            mac: msg.mac,
            hwid: msg.hwid,
            pic: msg.pic,
        }
    }
}

#[derive(Message)]
pub struct SelectCharWithPicResponseMessage {
    pub client_id: i32,
    pub char_id: i32,
    pub status: bool,
}
