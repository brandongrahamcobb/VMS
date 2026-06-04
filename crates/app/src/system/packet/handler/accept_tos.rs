/* tos/store.rs
 * The purpose of this module is to resolve relevant variables for Terms of Service.
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

use crate::message::packet::accept_tos::ReadTosRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::codec;
use crate::system::system_params::{InParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_tos(
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadTosRequestMessage>,
    command_tx: Res<CustomSender>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((_, acc, _)) = session_params.accounts.get(in_acc_entity) else {
            continue;
        };

        let accepted: bool = msg.confirmed as i16 == 0x01;

        if accepted {
            command_tx
                .0
                .lock()
                .unwrap()
                .send(AsyncCommand::DatabaseOperation(
                    DatabaseCommand::AcceptTosRequest {
                        client_id: msg.client_id,
                        acc_id: acc.id,
                    },
                ))
                .unwrap();
            let Ok(mut credentials_packet) =
                codec::login::builder::build_successful_login_packet(acc)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: credentials_packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        }
    }
}
