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

use crate::message::packet::list_worlds::ReadListWorldsRequestMessage;
use crate::message::result::HandlerResult;
use crate::system::packet::build::list_worlds;
use crate::system::system_params::LocationParams;
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::message::{MessageReader, MessageWriter};

pub fn handle_list_worlds(
    loc_params: LocationParams,
    mut messages: MessageReader<ReadListWorldsRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Ok(packets) =
            list_worlds::build_list_worlds_packets(&loc_params.worlds, &loc_params.channels)
        else {
            continue;
        };
        for mut packet in packets {
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        }
    }
}
