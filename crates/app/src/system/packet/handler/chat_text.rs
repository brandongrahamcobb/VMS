/* chat_text/store.rs
 * The purpose of this module is to resolve relevant variables during general chat.
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

use crate::component::character::MapleCharacter;
use crate::component::session::MapleSession;
use crate::message::packet::chat_text::ChatTextMessage;
use crate::resource::custom_resource::{ClientMap, Sessions};
use crate::system::packet::build::chat_text;
use crate::system::packet::handler::result::HandlerResult;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};
use net::packet::model::Packet;

pub async fn handle_chat_text(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<ChatTextMessage>,
    mut results: MessageWriter<HandlerResult>,
    accounts: Query<&MapleAccount>,
    chars: Query<&MapleCharacter>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(acc) = accounts.get(client_entity) else {
            continue;
        };
        let Ok(char) = chars.get(client_entity) else {
            continue;
        };

        let Ok(packet) = chat_text::build_chat_text_packet(acc.admin, char.id, msg.msg, msg.show)
        else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Broadcast(BroadcastAction::Send {
                packet: packet.clone(),
                scope: BroadcastScope::Map,
            })],
        });
    }
}
