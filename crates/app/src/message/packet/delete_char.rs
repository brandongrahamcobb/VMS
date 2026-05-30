/* delete_char/message.rs
 * The purpose of this module is to handle character deletion.
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
use ipc::data::delete_char::DeleteCharCommand;

#[derive(Message)]
pub struct DeleteCharMessage {
    pub client_id: i32,
    pub char_id: i32,
    pub pic: String,
}

impl From<(DeleteCharMessage, i32)> for DeleteCharCommand {
    fn from((msg, acc_id): (DeleteCharMessage, i32)) -> Self {
        Self {
            client_id: msg.client_id,
            acc_id,
            char_id: msg.char_id,
            pic: msg.pic,
        }
    }
}
