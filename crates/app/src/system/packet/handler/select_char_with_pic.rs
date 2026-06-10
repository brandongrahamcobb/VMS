/* select_char_with_pic/store.rs
 * The purpose of this module is to resolve relevant variables for PIC character selection.
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

use std::time::Instant;

use crate::component::character::InChar;
use crate::component::session::Transitioning;
use crate::message::packet::select_char_with_pic::{
    ReadSelectCharWithPicRequestMessage, SelectCharWithPicResponseMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{codec, spw};
use crate::system::system_params::{InParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Res};
use config::settings;
use inc::helpers;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_select_char_with_pic_request(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadSelectCharWithPicRequestMessage>,
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
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::SelectCharWithPicRequest {
                    client_id: msg.client_id,
                    acc_id: acc.id,
                    char_id: msg.char_id,
                    mac: msg.mac.clone(),
                    hwid: msg.hwid.clone(),
                    pic: msg.pic.clone(),
                },
            ))
            .unwrap()
    }
}

pub fn handle_select_char_with_pic_response(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut results: MessageWriter<HandlerResult>,
    mut messages: MessageReader<SelectCharWithPicResponseMessage>,
) -> () {
    for msg in messages.read() {
        if msg.status {
            let Ok(addr) = settings::get_routing_address() else {
                continue;
            };
            let octets: [u8; 4] = helpers::convert_to_ip_array(addr);

            let Some(&client_entity): Option<&Entity> = client_map.0.get(&msg.client_id) else {
                continue;
            };
            let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
                continue;
            };
            let Ok((_, channel, _)) = loc_params.channels.get(in_channel.0) else {
                continue;
            };
            let Ok(in_session) = in_params.in_sessions.get(client_entity) else {
                continue;
            };
            let Some((char_entity, char, _)) = session_params
                .chars
                .iter()
                .find(|(_, c, _)| c.id == msg.char_id)
            else {
                continue;
            };
            commands.entity(in_session.0).insert(Transitioning {
                map_wz: char.map_wz,
                started_at: Instant::now(),
            });
            commands.entity(client_entity).insert(InChar(char_entity));

            let Ok(mut select_char_packet) =
                codec::login::builder::build_select_char_packet(msg.char_id, octets, channel.port)
            else {
                continue;
            };

            command_tx
                .0
                .lock()
                .unwrap()
                .send(AsyncCommand::AcceptTransition {
                    client_id: msg.client_id,
                    port: channel.port,
                })
                .unwrap();

            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: select_char_packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        } else {
            let success_status: bool = false;
            let Ok(mut select_char_failed_packet) = spw::build_spw_packet(success_status) else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: select_char_failed_packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        }
    }
}
