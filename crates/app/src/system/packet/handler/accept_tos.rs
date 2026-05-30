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

use crate::component::account::MapleAccount;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::codec;
use crate::system::packet::handler::result::HandlerResult;
use crate::{message::packet::accept_tos::TosMessage, resource::custom_resource::Sessions};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use ipc::tcp_command::TcpCommand;

pub async fn handle_tos(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<TosMessage>,
    command_tx: CustomSender<TcpCommand>,
    mut results: MessageWriter<HandlerResult>,
    accounts: Query<&MapleAccount>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(acc) = accounts.get(client_entity) else {
            continue;
        };

        let accepted: bool = msg.confirmed == 0x01;

        if accepted {
            command_tx
                .0
                .send(TcpCommand::SetTosAccepted {
                    client_id: msg.client_id,
                    acc_id: acc.id,
                })
                .unwrap();
            let Ok(credentials_packet) =
                codec::login::builder::build_credentials_handler_successful_login_packet(acc)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::Session(SessionAction::Send {
                    packet: credentials_packet.finish(),
                    scope: SessionScope::Local,
                })],
            });
        }
    }
}
