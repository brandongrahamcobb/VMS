/* app/src/plugin/dispatch_plugin.rs
 * The purpose of this module is to provide a plugin recruiting packet read systems.
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

use crate::message::packet::raw::RawPacketMessage;
use crate::system::packet_dispatch;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::schedule::IntoScheduleConfigs;

pub struct PacketDispatchPlugin;

impl Plugin for PacketDispatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RawPacketMessage>().add_systems(
            Update,
            (
                packet_dispatch::login_packet_dispatch_system,
                packet_dispatch::channel_dispatch_system,
                packet_dispatch::char_management_dispatch_system,
                packet_dispatch::char_dispatch_system,
                packet_dispatch::item_dispatch_system,
                packet_dispatch::map_dispatch_system,
                packet_dispatch::move_dispatch_system,
                packet_dispatch::prepare_chars_dispatch_system,
                packet_dispatch::start_playing_dispatch_system,
                packet_dispatch::ui_dispatch_system,
            )
                .chain(),
        );
    }
}
