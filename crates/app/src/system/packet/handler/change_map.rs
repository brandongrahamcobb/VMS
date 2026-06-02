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
use bevy::ecs::system::{Commands, Query, Res};
use ipc::asyncronous::db_command::DatabaseCommand;

use crate::component::channel::{InChannel, MapleChannel};
use crate::component::character::{InChar, MapleCharacter};
use crate::component::map::{InMap, MapleMap};
use crate::component::session::{InSession, MapleSession};
use crate::message::packet::change_map::ChangeMapMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{change_map, codec};
use action::model::{Action, SessionAction};
use action::scope::{MapScope, SessionScope};

pub fn handle_map_change(
    mut commands: Commands,
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    mut sessions: Query<&mut MapleSession>,
    in_sessions: Query<(Entity, &InSession)>,
    channels: Query<&MapleChannel>,
    in_channels: Query<&InChannel>,
    maps: Query<&MapleMap>,
    in_maps: Query<(Entity, &InMap)>,
    portals: Query<(&MaplePortal, &ChildOf)>,
    chars: Query<&MapleCharacter>,
    in_chars: Query<(Entity, &InChar)>,
    mut messages: MessageReader<ChangeMapMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_session_entity, _)) = in_sessions.get(client_entity) else {
            continue;
        };
        let Ok(mut session) = sessions.get_mut(in_session_entity) else {
            continue;
        };
        let Ok((in_map_entity, _)) = in_maps.get(client_entity) else {
            continue;
        };
        let Some((portal, _)) = portals
            .iter()
            .find(|(p, parent)| p.target_name == msg.target_name && parent.0 == in_map_entity)
        else {
            continue;
        };
        let Ok((in_channel_entity, _)) = in_channels.get(client_entity) else {
            continue;
        };
        let Ok(channel) = channels.get(in_channel_entity) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_chars.get(client_entity) else {
            continue;
        };
        let Ok(char) = chars.get(in_char_entity) else {
            continue;
        };

        session.transitioning = true;
        commands.entity(client_entity).remove::<InMap>();

        let Some((map_entity, map, _)) = maps
            .iter()
            .find(|(_, m, parent)| m.wz = portal.target_map_wz && parent.0 == in_channel.0)
        else {
            continue;
        };

        commands.entity(client_entity).insert(InMap(map_entity));

        command_tx
            .0
            .lock()
            .unwrap()
            .send(DatabaseCommand::SetMap {
                client_id: msg.client_id,
                char_id: char.id,
                map_wz: map.wz,
            })
            .unwrap();

        let Ok(mut despawn_packet) = codec::player::builder::build_despawn_player_packet(char.id)
        else {
            continue;
        };
        let Ok(mut set_field_packet) =
            change_map::build_set_field_change_map_packet(channel.id, map.wz, portal.portal_wz)
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
