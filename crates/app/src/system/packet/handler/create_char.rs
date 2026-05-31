/* create_char/store.rs
 * The purpose of this module is to resolve relevant variables for character creation.
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

use crate::component::account::MapleAccount;
use crate::component::character::MapleCharacter;
use crate::component::world::InWorld;
use crate::message::packet::create_char::CreateCharMessage;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::create_char;
use bevy::ecs::message::MessageReader;
use bevy::ecs::system::{Commands, Query, Res};
use ipc::tcp_command::TcpCommand;

pub async fn handle_create_char_request(
    client_map: Res<ClientMap>,
    commands: Commands,
    mut messages: MessageReader<CreateCharRequestMessage>,
    command_tx: CustomSender<TcpCommand>,
    accounts: Query<&MapleAccount>,
    in_world: Query<&InWorld>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(acc) = accounts.get(client_entity) else {
            continue;
        };
        let Ok(in_world) = in_worlds.get(client_entity) else {
            continue;
        };
        commands_tx
            .0
            .send(TcpCommand::CreateChar::from((msg, acc.id, in_world.id)));
    }
}

pub async fn handle_create_char_response(
    client_map: Res<ClientMap>,
    commands: Commands,
    mut messages: MessageReader<CreateCharResponseMessage>,
    command_tx: CustomSender<TcpCommand>,
    accounts: Query<&MapleAccount>,
    in_world: Query<&InWorld>,
) -> () {
    for msg in messages.read() {
        let Ok(acc) = accounts.get(client_entity) else {
            continue;
        };
        let char: MapleCharacter = MapleCharacter::from(msg);
        commands.spawn((char, ChildOf(acc.0)));
        let Ok(create_char_packet) = create_char::build_create_char_packet(&char)?.finish();
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            })],
        });
    }
}
