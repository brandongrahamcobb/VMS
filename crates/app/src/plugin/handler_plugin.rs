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
use bevy::app::{App, Plugin, Update};

pub struct PacketDispatchPlugin;

impl Plugin for PacketDispatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handler::accept_tos::handle_tos)
            .add_systems(Update, handler::attack_close::handle_close_attack_request)
            .add_systems(Update, handler::attack_close::handle_close_attack_response)
            .add_systems(Update, handler::attack_close::handle_dead_mob)
            .add_systems(Update, handler::cc::handle_change_channel)
            .add_systems(Update, handler::change_keymap::handle_change_keymap)
            .add_systems(Update, handler::chat_text::handle_chat_text)
            .add_systems(
                Update,
                handler::check_char_name::handle_check_char_name_request,
            )
            .add_systems(
                Update,
                handler::check_char_name::handle_check_char_name_response,
            )
            .add_systems(Update, handler::create_char::handle_create_char_request)
            .add_systems(Update, handler::create_char::handle_create_char_response)
            .add_systems(Update, handler::delete_char::handle_delete_char_request)
            .add_systems(Update, handler::enter_cash_shop::handle_enter_cash_shop)
            .add_systems(Update, handler::list_chars::handle_load_char_slots)
            .add_systems(Update, handler::list_chars::handle_list_chars)
            .add_systems(Update, handler::list_worlds::handle_list_worlds)
            .add_systems(Update, handler::login::handle_login_failed_response)
            .add_systems(Update, handler::login::handle_login_request)
            .add_systems(Update, handler::login::handle_login_success_response)
            .add_systems(Update, handler::mob_moved::handle_mob_moved)
            .add_systems(Update, handler::pickup_item::handle_pickup_item_request)
            .add_systems(Update, handler::pickup_item::handle_pickup_response)
            .add_systems(
                Update,
                handler::player_logged_in::handle_player_logged_in_request,
            )
            .add_systems(
                Update,
                handler::player_logged_in::handle_player_logged_in_response,
            )
            .add_systems(
                Update,
                handler::player_map_transfer::handle_player_map_transfer,
            )
            .add_systems(Update, handler::player_moved::handle_player_moved)
            .add_systems(Update, handler::register_pic::handle_register_pic)
            .add_systems(Update, handler::select_char::handle_select_char)
            .add_systems(
                Update,
                handler::select_char_with_pic::handle_select_char_with_pic_request,
            )
            .add_systems(
                Update,
                handler::select_char_with_pic::handle_select_char_with_pic_response,
            )
            .add_systems(Update, handler::server_status::handle_server_status)
            .add_systems(Update, handler::take_damage::handle_take_damage);
    }
}
