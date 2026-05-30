/* app/src/system/event_handler.rs
 * The purpose of this module is to provide a system for handling plugin events.
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

use crate::message::packet::check_char_name::CheckCharNameResponseMessage;
use crate::message::packet::list_chars::{CharSlotsLoadedMessage, ListCharsSuccessMessage};
use crate::plugin::event::CustomPluginEvent;
use crate::resource::custom_resource::CustomReceiver;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Res;
use ipc::tcp_event::TcpEvent;

pub fn handle_events_system(
    receiver: Res<CustomReceiver>,
    mut check_char_name_writer: MessageWriter<CheckCharNameResponseMessage>,
    mut list_chars_success_writer: MessageWriter<ListCharsSuccessMessage>,
    mut list_chars_failed_writer: MessageWriter<ListCharsFailedMessage>,
) {
    let rx: MutexGuard<Receiver<TcpEvent>> = receiver.0.lock().unwrap();
    while let Ok(event) = rx.try_recv() {
        match event {
            TcpEvent::ClientConnected { client_id } => {}
            TcpEvent::ClientDisconnected { client_id } => {}
            TcpEvent::PacketReceived { client_id, packet } => {}
            TcpEvent::ListCharsSuccess {
                client_id,
                channel_id,
                chars,
                slots,
                world_id,
            } => {
                list_chars_success_writer.write(ListCharsSuccessMessage {
                    client_id,
                    channel_id,
                    chars,
                    slots,
                    world_id,
                });
            }
            TcpEvent::ListCharsFailed { client_id } => {
                list_chars_failure_wrtier.write(ListCharsFailedMessage { client_id });
            }
            TcpEvent::CheckCharName {
                client_id,
                exists,
                ign,
            } => {
                check_char_name_writer.write(CheckCharNameResponseMessage {
                    client_id,
                    exists,
                    ign,
                });
            }
        }
    }
}
