/* player_logged_in/store.rs
 * The purpose of this module is to resolve relevant variables for player login.
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
use crate::component::character::{InChar, MapleCharacter};
use crate::component::map::{InMap, MapleMap};
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{codec, player_logged_in};
use crate::system::packet::handler::result::HandlerResult;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_player_logged_in_request(
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    chars: Query<&MapleCharacter>,
    in_chars: Query<(Entity, &InChar)>,
    mut messages: MessageReader<PlayerLoggedInRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_chars.get(client_entity) else {
            continue;
        };
        let Ok(char) = chars.get(in_char_entity) else {
            continue;
        };

        command_tx
            .0
            .lock()
            .unwrap()
            .send(DatabaseCommand::JoinPlayer {
                client_id: msg.client_id,
                char_id: char.id,
            })
            .unwrap();
    }
}

pub fn handle_player_logged_in_response(
    commands: Commands,
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    channels: Query<&MapleChannel>,
    in_channels: Query<(Entity, &InChannel)>,
    maps: Query<&MapleMap>,
    in_maps: Query<(Entity, &InMap)>,
    chars: Query<(Entity, &MapleCharacter)>,
    in_chars: Query<(Entity, &InChar)>,
    mut messages: MessageReader<PlayerLoggedInResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_channel_entity, _)) = in_channels.get(client_entity) else {
            continue;
        };
        let Ok(channel) = channels.get(in_channel_entity) else {
            continue;
        };
        let Ok((in_map_entity, _)) = in_maps.get(client_entity) else {
            continue;
        };
        let Ok(map) = maps.get(in_map_entity) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_chars.get(client_entity) else {
            continue;
        };
        let Ok((char_entity, char)) = chars.get(in_char_entity) else {
            continue;
        };

        commands.spawn((MapleKeybindings::from(msg.binds), ChildOf(char_entity)));
        commands.spawn((MapleKeybindings::from(msg.binds), ChildOf(char_entity)));

        let Ok(keymap_packet) = player_logged_in::build_player_logged_in_keymap_packet(&msg.binds)
        else {
            continue;
        };
        let Ok(mut set_field_packet) =
            codec::player::builder::build_set_field_packet(&char, channel.id, map.wz)
        else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![
                Action::Session(SessionAction::Send {
                    packet: keymap_packet.finish(),
                    scope: SessionScope::Local,
                }),
                Action::Session(SessionAction::Send {
                    packet: set_field_packet.finish(),
                    scope: SessionScope::Local,
                }),
                Action::Session(SessionAction::Retrieve),
            ],
        });
    }
}
