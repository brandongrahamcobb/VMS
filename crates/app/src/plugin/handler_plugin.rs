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

use crate::system::packet::handler;
use crate::system::result_handler;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::schedule::IntoScheduleConfigs;

pub struct LoginPacketHandlerPlugin;

impl Plugin for LoginPacketHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handler::accept_tos::handle_tos,
                handler::check_char_name::handle_check_char_name_request,
                handler::check_char_name::handle_check_char_name_response,
                handler::create_char::handle_create_char_request,
                handler::create_char::handle_create_char_response,
                handler::delete_char::handle_delete_char_request,
                handler::list_chars::handle_load_char_slots,
                handler::list_chars::handle_list_chars,
                handler::list_worlds::handle_list_worlds,
                handler::login::handle_login_failed_response,
                handler::login::handle_login_request,
                handler::login::handle_login_success_response,
                handler::player_logged_in::handle_player_logged_in_request,
                handler::player_logged_in::handle_player_logged_in_response,
                handler::register_pic::handle_register_pic,
                handler::select_char::handle_select_char,
                handler::select_char_with_pic::handle_select_char_with_pic_request,
                handler::select_char_with_pic::handle_select_char_with_pic_response,
                handler::server_status::handle_server_status,
            )
                .chain(),
        );
    }
}

pub struct GamePacketHandlerPlugin;

impl Plugin for GamePacketHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handler::attack_close::handle_close_attack_request,
                handler::attack_close::handle_close_attack_response,
                handler::attack_close::handle_dead_mob,
                handler::cc::handle_change_channel,
                handler::change_keymap::handle_change_keymap,
                handler::chat_text::handle_chat_text,
                handler::enter_cash_shop::handle_enter_cash_shop,
                handler::mob_moved::handle_mob_moved,
                handler::pickup_item::handle_pickup_item_request,
                handler::pickup_item::handle_pickup_response,
                handler::player_map_transfer::handle_player_map_transfer,
                handler::player_moved::handle_player_moved,
                handler::take_damage::handle_take_damage,
            )
                .chain(),
        );
    }
}
