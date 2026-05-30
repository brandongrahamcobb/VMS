/* tos/message.rs
 * The purpose of this module is to handle Terms of Service acceptance.
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
use core::convert::From;
use ipc::data::accept_tos::TosCommand;

#[derive(Message)]
pub struct TosMessage {
    pub client_id: i32,
    pub confirmed: bool,
}

impl From<(TosMessage, i32)> for TosCommand {
    fn from((msg, acc_id): (TosMessage, i32)) -> Self {
        Self {
            client_id: msg.client_id,
            acc_id,
            confirmed: msg.confirmed,
        }
    }
}
