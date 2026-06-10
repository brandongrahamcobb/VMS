/* register_pic/store.rs
 * The purpose of this module is to resolve relevant variables for PIC registration.
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

use crate::message::packet::register_pic::ReadRegisterPicRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::spw;
use crate::system::system_params::{InParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_register_pic(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadRegisterPicRequestMessage>,
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

        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::ChangePicRequest {
                    client_id: msg.client_id,
                    acc_id: acc.id,
                    pic: msg.pic.clone(),
                },
            ))
            .unwrap();

        let success_status: bool = true;
        let Ok(mut spw_packet) = spw::build_spw_packet(success_status) else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: spw_packet.finish(),
                scope: ActionScope::Local,
            }],
        });
    }
}
