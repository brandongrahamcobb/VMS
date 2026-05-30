/* cc/store.rs
 * The purpose of this module is to resolve relevant variables for changing channels.
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

use crate::component::channel::{InChannel, MapleChannel};
use crate::component::session::MapleSession;
use crate::component::world::{InWorld, MapleWorld};
use crate::message::packet::cc::ChangeChannelMessage;
use crate::resource::custom_resource::{ClientMap, Sessions};
use crate::system::packet::build::{cc, codec};
use crate::system::packet::handler::result::HandlerResult;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use bevy::ecs::system::{Commands, Query};
use bevy::gizmos::circles;
use config::settings;
use inc::helpers;
use net::packet::model::Packet;
use session::model::Session;

pub fn handle_change_channel(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    mut messages: MessageReader<ChangeChannelMessage>,
    mut results: MessageWriter<HandlerResult>,
    in_world: Query<&InWorld>,
    channels: Query<(Entity, &MapleChannel, &ChildOf)>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        commands.entity(client_entity).remove::<InChannel>();
        let Ok(in_world) = in_world.get(client_entity) else {
            continue;
        };
        let Some((channel_entity, channel, _)) = channels
            .iter()
            .find(|(c, parent)| c.id == msg.channel_id && parent.0 == in_world.0)
        else {
            continue;
        };
        commands
            .entity(client_entity)
            .insert(InChannel(channel_entity));
        let Ok(char) = characters.get(client_entity) else {
            continue;
        };

        let Ok(addr) = settings::get_routing_address() else {
            continue;
        };
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);

        let Ok(despawn_packet) = codec::player::builder::build_despawn_player_packet(char.id)
        else {
            continue;
        };
        let Ok(cc_packet) = cc::build_channel_change_packet(octets, channel.port) else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![
                Action::Session(SessionAction::Send {
                    packet: despawn_packet.finish(),
                    scope: SessionScope::Map(MapScope::SameChannelSameWorld),
                }),
                result.add_action(Action::Session(SessionAction::Break {
                    packet: cc_packet.finish(),
                })),
            ],
        });
    }
}
