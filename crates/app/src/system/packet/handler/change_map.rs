/* change_map/store.rs
 * The purpose of this module is to resolve relevant variables when changing maps.
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
use ipc::tcp_command::AsyncCommand;
use net::packet::model::Packet;

use crate::component::channel::InChannel;
use crate::component::character::MapleCharacter;
use crate::component::map::InMap;
use crate::component::session::MapleSession;
use crate::message::packet::change_map::ChangeMapMessage;
use crate::resource::custom_resource::{ClientMap, CustomSender, Sessions};
use crate::system::packet::build::{change_map, codec};
use crate::system::packet::handler::result::HandlerResult;

pub async fn handle_map_change(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    mut messages: MessageReader<ChangeMapMessage>,
    command_tx: CustomSender<AsyncCommand>,
    mut results: MessageWriter<HandlerResult>,
    chars: Query<&MapleCharacter>,
    channels: Query<&MapleChannel>,
    in_channel: Query<&InChannel>,
    in_map: Query<&InMap>,
    portals: Query<(&MaplePortal, &ChildOf)>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_map) = in_map.get(client_entity) else {
            continue;
        };
        let Some((portal, _)) = portals
            .iter()
            .find(|(p, parent)| p.target_name == msg.target_name && parent.0 == in_map_entity.0)
        else {
            continue;
        };
        commands.entity(client_entity).remove::<InMap>();
        let Ok(in_channel) = in_channel.get(client_entity) else {
            continue;
        };
        let Ok((map_entity, map, _)) = maps
            .iter()
            .find(|(_, m, parent)| m.wz = portal.target_map_wz && parent.0 == in_channel.0)
        else {
            continue;
        };
        commands.entity(client_entity).insert(InMap(map_entity));
        let Ok(char) = chars.get(client_entity) else {
            continue;
        };

        command_tx
            .0
            .send(AsyncCommand::SetMap {
                client_id: msg.client_id,
                char_id: char.id,
                map_wz: map.wz,
            })
            .unwrap();

        let Ok(despawn_packet) = codec::player::builder::build_despawn_player_packet(char.id)
        else {
            continue;
        };
        let Ok(set_field_packet) =
            change_map::build_set_field_change_map_packet(in_channel.id, map.wz, portal.portal_wz)
        else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![
                Action::Session(SessionAction::Send {
                    packet: despawn_packet.finish(),
                    scope: SessionScope::Map(MapScope::SameChannelSameWorld),
                }),
                Action::Session(SessionAction::Send {
                    packet: set_field_packet.finish(),
                    scope: SessionScope::Local,
                }),
            ],
        });
    }
}
