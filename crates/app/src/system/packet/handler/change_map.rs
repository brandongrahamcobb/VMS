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

use std::time::Instant;

use crate::component::map::InMap;
use crate::component::portal::MaplePortal;
use crate::component::session::Transitioning;
use crate::message::packet::change_map::ReadChangeMapRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{change_map, codec};
use crate::system::system_params::{InParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::{ActionScope, MapScope};
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_map_change(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: &mut SessionParams,
    portals: Query<(&MaplePortal, &ChildOf)>,
    mut messages: MessageReader<ReadChangeMapRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Some((session_entity, _, _)) = session_params
            .sessions
            .iter_mut()
            .find(|(_, _, parent)| parent.0 == client_entity)
        else {
            continue;
        };
        commands.entity(session_entity).insert(Transitioning {
            started_at: Instant::now(),
        });
        let Ok(in_map) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        commands.entity(client_entity).remove::<InMap>();
        let Some((portal, _)) = portals.iter().find(|(p, parent)| {
            p.base.target_portal_name == msg.target_name && parent.0 == in_map.0
        }) else {
            continue;
        };
        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((_, channel, _)) = loc_params.channels.get(in_channel.0) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };

        let Some((map_entity, map, _)) = loc_params.maps.iter().find(|(_, m, parent)| {
            m.base.wz == portal.base.target_map_wz && parent.0 == in_channel.0
        }) else {
            continue;
        };
        commands.entity(client_entity).insert(InMap(map_entity));

        command_tx
            .0
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::UpdateMapRequest {
                    client_id: msg.client_id,
                    char_id: char.id,
                    map_wz: map.base.wz,
                },
            ))
            .unwrap();

        let Ok(mut despawn_packet) = codec::player::builder::build_despawn_player_packet(char.id)
        else {
            continue;
        };
        let Ok(mut set_field_packet) =
            change_map::build_set_field_change_map_packet(channel.id, map.base.wz, portal.base.wz)
        else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![
                Action::HandlerAction {
                    packet: despawn_packet.finish(),
                    scope: ActionScope::Map(MapScope::SameChannelSameWorld),
                },
                Action::HandlerAction {
                    packet: set_field_packet.finish(),
                    scope: ActionScope::Local,
                },
            ],
        });
    }
}
