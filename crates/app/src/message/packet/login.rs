/* credentials/message.rs
 * The purpose of this module is to handle credentials validation.
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

use base::account::FailedCode;
use bevy::prelude::Message;
use db::account::model::AccountModel;

#[derive(Message)]
pub struct ReadLoginRequestMessage {
    pub client_id: i32,
    pub username: String,
    pub pw: String,
    pub hwid: String,
}

#[derive(Message)]
pub struct LoginSuccessResponseMessage {
    pub client_id: i32,
    pub acc_id: i32,
    pub acc_model: AccountModel,
}

#[derive(Message)]
pub struct LoginFailedResponseMessage {
    pub client_id: i32,
    pub code: FailedCode,
}
