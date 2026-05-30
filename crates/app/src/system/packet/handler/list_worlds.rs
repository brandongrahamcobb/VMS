/* list_worlds/handler.rs
 * The purpose of this module is to handle world listing.
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

use crate::component::world::MapleWorld;
use crate::message::packet::list_worlds::ListWorldsMessage;
use crate::result::HandlerResult;
use crate::system::packet::handler::result::HandlerResult;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Query;
use net::packet::build::list_worlds;
use net::packet::model::Packet;

async fn handle_list_worlds(
    mut messsages: MessageReader<ListWorldsMessage>,
    mut results: MessageWriter<HandlerResult>,
    worlds: Query<&MapleWorld>,
) -> () {
    for msg in messages.read() {
        let Ok(packets) = list_worlds::build_list_worlds_packets(&worlds) else {
            continue;
        };
        for packet in packets {
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::Session(SessionAction::Send {
                    packet.finish(),
                    scope: SessionScope::Local,
                })],
            });
        }
    }
}
