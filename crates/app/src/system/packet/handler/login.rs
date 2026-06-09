/* app/src/system/packet/handler/login.rs
 * The purpose of this module is to process a login request and response.
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
use crate::component::account::InAccount;
use crate::component::account::MapleAccount;
use crate::message::packet::login::LoginFailedResponseMessage;
use crate::message::packet::login::LoginSuccessResponseMessage;
use crate::message::packet::login::ReadLoginRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::resource::custom_resource::CustomSender;
use crate::system::packet::build::codec;
use crate::system::system_params::InParams;
use crate::system::system_params::SessionParams;
use action::model::Action;
use action::scope::ActionScope;
use base::account::FailedCode;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::MessageReader;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Res;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_login_request(
    command_tx: Res<CustomSender>,
    session_params: SessionParams,
    mut messages: MessageReader<ReadLoginRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) {
    for msg in messages.read() {
        let already_logged_in = session_params
            .accounts
            .iter()
            .find(|(_, a, _)| a.username == msg.username);
        if already_logged_in.is_some() {
            let Ok(mut login_failed_packet) =
                codec::login::builder::build_failed_login_packet(FailedCode::Playing as i16)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: login_failed_packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        }
        command_tx
            .0
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::LoginRequest {
                    client_id: msg.client_id,
                    username: msg.username.clone(),
                    password: msg.pw.clone(),
                },
            ))
            .unwrap();
    }
}

pub fn handle_login_success_response(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    in_params: InParams,
    mut messages: MessageReader<LoginSuccessResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_session) = in_params.in_sessions.get(client_entity) else {
            continue;
        };
        let acc: MapleAccount = MapleAccount::from((msg.acc_model.clone(), msg.acc_id));
        let acc_entity = commands.spawn((acc.clone(), ChildOf(in_session.0))).id();
        commands.entity(client_entity).insert(InAccount(acc_entity));
        let Ok(mut credentials_packet) = codec::login::builder::build_successful_login_packet(&acc)
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

pub fn handle_login_failed_response(
    mut messages: MessageReader<LoginFailedResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) {
    for msg in messages.read() {
        let Ok(mut login_failed_packet) =
            codec::login::builder::build_failed_login_packet(msg.code.clone() as i16)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: login_failed_packet.finish(),
                scope: ActionScope::Local,
            }],
        });
    }
}
