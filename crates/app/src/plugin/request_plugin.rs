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

use crate::message::packet::accept_tos::ReadTosRequestMessage;
use crate::message::packet::attack_close::ReadCloseAttackRequestMessage;
use crate::message::packet::cc::ReadChangeChannelRequestMessage;
use crate::message::packet::change_keymap::ReadChangeKeymapRequestMessage;
use crate::message::packet::change_map::ReadChangeMapRequestMessage;
use crate::message::packet::chat_text::ReadChatTextRequestMessage;
use crate::message::packet::check_char_name::ReadCheckCharNameRequestMessage;
use crate::message::packet::create_char::ReadCreateCharRequestMessage;
use crate::message::packet::delete_char::ReadDeleteCharRequestMessage;
use crate::message::packet::enter_cash_shop::ReadEnterCashShopRequestMessage;
use crate::message::packet::list_chars::ReadListCharsRequestMessage;
use crate::message::packet::list_worlds::ReadListWorldsRequestMessage;
use crate::message::packet::login::ReadLoginRequestMessage;
use crate::message::packet::login_started::ReadLoginStartRequestMessage;
use crate::message::packet::mob_moved::ReadMobMovedRequestMessage;
use crate::message::packet::party_search::ReadPartySearchRequestMessage;
use crate::message::packet::pickup_item::ReadPickupItemRequestMessage;
use crate::message::packet::player_logged_in::ReadPlayerLoggedInRequestMessage;
use crate::message::packet::player_map_transferred::ReadPlayerMapTransferRequestMessage;
use crate::message::packet::player_moved::ReadPlayerMovedRequestMessage;
use crate::message::packet::register_pic::ReadRegisterPicRequestMessage;
use crate::message::packet::select_char::ReadSelectCharRequestMessage;
use crate::message::packet::select_char_with_pic::ReadSelectCharWithPicRequestMessage;
use crate::message::packet::server_status::ReadServerStatusRequestMessage;
use crate::message::packet::spw::ReadSpwRequestMessage;
use crate::message::packet::take_damage::ReadTakeDamageRequestMessage;
use bevy::app::{App, Plugin};

pub struct RequestPlugin;

impl Plugin for RequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ReadTosRequestMessage>()
            .add_message::<ReadCloseAttackRequestMessage>()
            .add_message::<ReadChangeChannelRequestMessage>()
            .add_message::<ReadChangeKeymapRequestMessage>()
            .add_message::<ReadChangeMapRequestMessage>()
            .add_message::<ReadChatTextRequestMessage>()
            .add_message::<ReadCheckCharNameRequestMessage>()
            .add_message::<ReadCreateCharRequestMessage>()
            .add_message::<ReadDeleteCharRequestMessage>()
            .add_message::<ReadEnterCashShopRequestMessage>()
            .add_message::<ReadListCharsRequestMessage>()
            .add_message::<ReadListWorldsRequestMessage>()
            .add_message::<ReadLoginRequestMessage>()
            .add_message::<ReadLoginStartRequestMessage>()
            .add_message::<ReadMobMovedRequestMessage>()
            .add_message::<ReadPartySearchRequestMessage>()
            .add_message::<ReadPickupItemRequestMessage>()
            .add_message::<ReadPlayerLoggedInRequestMessage>()
            .add_message::<ReadPlayerMapTransferRequestMessage>()
            .add_message::<ReadPlayerMovedRequestMessage>()
            .add_message::<ReadRegisterPicRequestMessage>()
            .add_message::<ReadSelectCharRequestMessage>()
            .add_message::<ReadSelectCharWithPicRequestMessage>()
            .add_message::<ReadServerStatusRequestMessage>()
            .add_message::<ReadSpwRequestMessage>()
            .add_message::<ReadTakeDamageRequestMessage>();
    }
}
