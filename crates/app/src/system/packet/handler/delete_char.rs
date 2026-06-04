/* delete_char/store.rs
 * The purpose of this module is to resolve relevant variables for character deletion.
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

use crate::message::packet::delete_char::ReadDeleteCharRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::spw;
use crate::system::system_params::{InParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Res};
use config::settings;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;
use ipc::syncronous;

pub fn handle_delete_char_request(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadDeleteCharRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let mut pic_status = false;
        let Ok(use_pic) = settings::get_pic_required() else {
            continue;
        };

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((acc_entity, acc, _)) = session_params.accounts.get(in_acc_entity) else {
            continue;
        };
        let Some((char_entity, _, _)) = session_params
            .chars
            .iter()
            .find(|(_, c, parent)| c.id == msg.char_id && parent.0 == acc_entity)
        else {
            continue;
        };

        if use_pic {
            pic_status = syncronous::account::check_pic(acc.pic.clone(), msg.pic.clone());
        }
        if !pic_status {
            commands.entity(char_entity).despawn();
            command_tx
                .0
                .lock()
                .unwrap()
                .send(AsyncCommand::DatabaseOperation(
                    DatabaseCommand::DeleteCharRequest {
                        client_id: msg.client_id,
                        char_id: msg.char_id,
                    },
                ))
                .unwrap();
            let Ok(mut spw_packet) = spw::build_spw_packet(!pic_status) else {
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
}
