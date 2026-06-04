/* player_map_transfer/store.rs
 * The purpose of this module is to resolve relevant variables for player map transfers.
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

use action::model::{Action, SessionAction};
use action::scope::{MapScope, SessionScope};
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};

use crate::component::item::MapleItem;
use crate::message::packet::player_map_transferred::ReadPlayerMapTransferRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::codec;
use crate::system::system_params::{InParams, InventoryParams, SessionParams};

pub fn handle_player_map_transfer(
    client_map: Res<ClientMap>,
    in_params: InParams,
    mut session_params: SessionParams,
    inv_params: InventoryParams,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<ReadPlayerMapTransferRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((_, mut session)) = session_params.sessions.get_mut(client_entity) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((char_entity, char, _)) = session_params.chars.get(in_char_entity) else {
            continue;
        };
        let Ok((inv_entity, _)) = inv_params.inventories.get(char_entity) else {
            continue;
        };
        let items: Vec<_> = items
            .iter()
            .filter(|(_, parent)| parent.0 == inv_entity)
            .collect();

        session.transitioning = false;

        let Ok(mut spawn_player_packet) =
            codec::player::builder::build_spawn_player_packet(char, items)
        else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: spawn_player_packet.finish(),
                scope: SessionScope::Map(MapScope::SameChannelSameWorld),
            })],
        });
    }
}
