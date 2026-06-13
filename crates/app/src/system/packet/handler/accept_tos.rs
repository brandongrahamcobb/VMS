/* app/src/system/handler/accept_tos.rs
 * The purpose of this module is to handle Terms of Service acceptance system messages.
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
use crate::system::packet::handler::result::accept_tos_result;
use crate::system::system_params::{InParams, SessionParams};
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

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
        let Ok(in_acc) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((_, acc, _)) = session_params.accounts.get(in_acc.0) else {
            continue;
        };

        let accepted: bool = msg.confirmed as i16 == 0x01;

        if accepted {
            command_tx
                .0
                .send(AsyncCommand::DatabaseOperation(
                    DatabaseCommand::AcceptTosRequest {
                        client_id: msg.client_id,
                        acc_id: acc.id,
                    },
                ))
                .unwrap();
            accept_tos_result::write_result(msg.client_id, &vec![acc.clone()], &mut results);
        }
    }
}
