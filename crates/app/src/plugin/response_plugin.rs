/* app/src/plugin/custom_plugin.rs
 * The purpose of this module is to cross the thread boundary between Bevy and the TCP server.
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

use crate::message::packet::attack_close::{CloseAttackResponseMessage, DeadMobResponseMessage};
use crate::message::packet::check_char_name::CheckCharNameResponseMessage;
use crate::message::packet::create_char::CreateCharResponseMessage;
use crate::message::packet::list_chars::{
    ListCharsFailedResponseMessage, ListCharsSuccessResponseMessage,
};
use crate::message::packet::login::{LoginFailedResponseMessage, LoginSuccessResponseMessage};
use crate::message::packet::pickup_item::PickupItemResponseMessage;
use crate::message::packet::player_logged_in::PlayerLoggedInResponseMessage;
use crate::message::packet::select_char_with_pic::SelectCharWithPicResponseMessage;

use bevy::app::{App, Plugin};

pub struct ResponsePlugin;

impl Plugin for ResponsePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CloseAttackResponseMessage>()
            .add_message::<DeadMobResponseMessage>()
            .add_message::<CheckCharNameResponseMessage>()
            .add_message::<CreateCharResponseMessage>()
            .add_message::<ListCharsSuccessResponseMessage>()
            .add_message::<ListCharsFailedResponseMessage>()
            .add_message::<LoginSuccessResponseMessage>()
            .add_message::<LoginFailedResponseMessage>()
            .add_message::<PickupItemResponseMessage>()
            .add_message::<PlayerLoggedInResponseMessage>()
            .add_message::<SelectCharWithPicResponseMessage>();
    }
}
