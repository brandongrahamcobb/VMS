/* server_status/handler.rs
 * The purpose of this module is to handle server status updates.
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

use crate::component::channel::MapleChannel;
use crate::message::packet::server_status::ReadServerStatusRequestMessage;
use crate::message::result::HandlerResult;
use crate::system::packet::build::server_status;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Query;

fn handle_server_status(
    mut messages: MessageReader<ReadServerStatusRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
    channels: Query<&MapleChannel>,
) {
    for msg in messages.read() {
        let status: i16 = if channels.iter().next().is_some() {
            0
        } else {
            2
        };
        let Ok(mut status_packet) = server_status::build_server_status_packet(status) else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: status_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}
