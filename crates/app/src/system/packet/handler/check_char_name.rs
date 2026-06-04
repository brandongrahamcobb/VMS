/* check_char_name/store.rs
 * The purpose of this module is to resolve relevant variables checking a character name.
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

use crate::message::packet::check_char_name::{
    CheckCharNameResponseMessage, ReadCheckCharNameRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::CustomSender;
use crate::system::packet::build::check_char_name;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_check_char_name_request(
    mut messages: MessageReader<ReadCheckCharNameRequestMessage>,
    command_tx: Res<CustomSender>,
) -> () {
    for msg in messages.read() {
        command_tx
            .0
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::CharNameRequest {
                    client_id: msg.client_id,
                    ign: msg.ign.clone(),
                },
            ))
            .unwrap();
    }
}

pub fn handle_check_char_name_response(
    mut messages: MessageReader<CheckCharNameResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Ok(mut check_char_name_packet) =
            check_char_name::build_check_char_name_packet(msg.exists, msg.ign.clone())
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: check_char_name_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}
