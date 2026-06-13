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

use crate::component::item::MapleItem;
use crate::message::packet::player_logged_in::{
    PlayerLoggedInResponseMessage, ReadPlayerLoggedInRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{codec, player_logged_in};
use crate::system::packet::handler::codec::load_char;
use crate::system::system_params::{InParams, InventoryParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_player_logged_in_request(
    command_tx: Res<CustomSender>,
    mut messages: MessageReader<ReadPlayerLoggedInRequestMessage>,
) -> () {
    for msg in messages.read() {
        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::JoinRequest {
                    client_id: msg.client_id,
                    char_id: msg.char_id,
                },
            ))
            .unwrap();
    }
}

pub fn handle_player_logged_in_response(
    client_map: Res<ClientMap>,
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
        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((_, channel, _)) = loc_params.channels.get(in_channel.0) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let char_map = load_char::load_char_with_equips(
            vec![client_entity],
            &in_params,
            &session_params,
            &inv_params,
            items,
        );
        let mut binds: Vec<_> = session_params
            .keybindings
            .iter()
            .filter(|(_, _, parent)| parent.0 == in_char.0)
            .collect();
        binds.sort_by_key(|(_, k, _)| k.key);

        let Ok(mut keymap_packet) = player_logged_in::build_player_logged_in_keymap_packet(&binds)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: keymap_packet.finish(),
                scope: ActionScope::Local,
            }],
        });

        for (char, equips) in char_map.iter() {
            let Ok(mut set_field_packet) =
                codec::player::set_field::build_set_field_packet(char, equips, channel.id)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: set_field_packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        }
    }
}
