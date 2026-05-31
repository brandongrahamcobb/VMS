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

use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};

use crate::component::channel::MapleChannel;
use crate::component::session::MapleSession;
use crate::component::world::MapleWorld;
use crate::message::packet::player_map_transferred::PlayerMapTransferMessage;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::codec;
use crate::system::packet::handler::result::HandlerResult;

pub async fn handle_player_map_transfer(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<PlayerMapTransferMessage>,
    mut results: MessageWriter<HandlerResult>,
    mut sessions: Query<&mut MapleSession>,
) -> () {
    for msg in messages.read() {
        let Some(client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Some(session) = sessions.get_mut(client_entity) else {
            continue;
        };
        let Some(char) = chars.get(client_entity) else {
            continue;
        };

        session.transitioning = false;

        let Some(spawn_player_packet): Option<Packet> =
            codec::player::builder::build_spawn_player_packet(char)
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
