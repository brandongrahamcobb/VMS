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

use crate::component::character::InChar;
use crate::component::item::MapleItem;
use crate::message::packet::player_logged_in::{
    PlayerLoggedInResponseMessage, ReadPlayerLoggedInRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{codec, player_logged_in};
use crate::system::system_params::{InParams, InventoryParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_player_logged_in_request(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadPlayerLoggedInRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char_entity) else {
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
    mut commands: Commands,
    client_map: Res<ClientMap>,
    parents: Query<&ChildOf>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    inv_params: InventoryParams,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<PlayerLoggedInResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_channel_entity, _)) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((_, channel, _)) = loc_params.channels.get(in_channel_entity) else {
            continue;
        };
        let Ok((in_map_entity, _)) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Ok((_, map, _)) = loc_params.maps.get(in_map_entity) else {
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
        let binds: Vec<_> = session_params
            .keybindings
            .iter()
            .filter(|(_, _, parent)| parent.0 == char_entity)
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
                Action::HandlerAction {
                    packet: keymap_packet.finish(),
                    scope: ActionScope::Local,
                },
                Action::HandlerAction {
                    packet: set_field_packet.finish(),
                    scope: ActionScope::Local,
                },
                // Action::HandlerAction(SessionAction::Retrieve), TODO
            ],
        });
    }
}
