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
use crate::component::inventory::MapleInventory;
use crate::component::item::MapleItem;
use crate::component::keybinding::MapleKeybinding;
use crate::component::map::{InMap, MapleMap};
use crate::message::packet::player_logged_in::{
    PlayerLoggedInResponseMessage, ReadPlayerLoggedInRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{codec, player_logged_in};
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_player_logged_in_request(
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    chars: Query<&MapleCharacter>,
    in_chars: Query<(Entity, &InChar)>,
    mut messages: MessageReader<ReadPlayerLoggedInRequestMessage>,
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
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::JoinRequest {
                    client_id: msg.client_id,
                    char_id: char.id,
                },
            ))
            .unwrap();
    }
}

pub fn handle_player_logged_in_response(
    commands: &mut Commands,
    client_map: Res<ClientMap>,
    parents: Query<&ChildOf>,
    channels: Query<&MapleChannel>,
    in_channels: Query<(Entity, &InChannel)>,
    maps: Query<&MapleMap>,
    in_maps: Query<(Entity, &InMap)>,
    chars: Query<(Entity, &MapleCharacter)>,
    in_chars: Query<(Entity, &InChar)>,
    inventories: Query<(Entity, &MapleInventory)>,
    items: Query<(&MapleItem, &ChildOf)>,
    binds: Query<(&MapleKeybinding, &ChildOf)>,
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
        let Ok((inv_entity, _)) = inventories.get(char_entity) else {
            continue;
        };
        commands.entity(client_entity).insert(InChar(char_entity));
        let items: Vec<_> = items
            .iter()
            .filter(|(_, parent)| {
                parents
                    .get(parent.parent())
                    .map(|tab_parent| tab_parent.parent() == inv_entity)
                    .unwrap_or(false)
            })
            .collect();
        let binds: Vec<_> = binds
            .iter()
            .filter(|(_, parent)| parent.0 == char_entity)
            .collect();

        let Ok(mut keymap_packet) = player_logged_in::build_player_logged_in_keymap_packet(&binds)
        else {
            continue;
        };
        let Ok(mut set_field_packet) =
            codec::player::builder::build_set_field_packet(&char, items, channel.id, map.base.wz)
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
