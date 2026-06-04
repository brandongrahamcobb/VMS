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

use crate::message::packet::chat_text::ReadChatTextRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::chat_text;
use crate::system::system_params::{InParams, SessionParams};
use action::model::{Action, BroadcastAction};
use action::scope::BroadcastScope;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;

pub fn handle_chat_text(
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadChatTextRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((_, acc, _)) = session_params.accounts.get(in_acc_entity) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char_entity) else {
            continue;
        };

        let Ok(mut chat_packet) =
            chat_text::build_chat_text_packet(acc.admin, char.id, msg.msg.clone(), msg.show)
        else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Broadcast(BroadcastAction::Send {
                packet: chat_packet.finish(),
                scope: BroadcastScope::Map,
            })],
        });
    }
}
