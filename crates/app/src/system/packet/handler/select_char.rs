/* app/src/system/packet/handler/select_char.rs
 * The purpose of this module is to handle pic-disabled character selection system messages.
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
use crate::message::packet::select_char::ReadSelectCharRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::handler::result::select_char_result;
use crate::system::system_params::{InParams, LocationParams, SessionParams};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Res};
use ipc::command::AsyncCommand;

pub fn handle_select_char(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut results: MessageWriter<HandlerResult>,
    mut messages: MessageReader<ReadSelectCharRequestMessage>,
) -> () {
    for msg in messages.read() {
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
            map_wz: char.spawn_map_wz,
            started_at: Instant::now(),
        });

        commands.entity(client_entity).insert(InChar(char_entity));

        command_tx
            .0
            .send(AsyncCommand::AcceptTransition {
                client_id: msg.client_id,
                port: channel.port,
            })
            .unwrap();

        select_char_result::write_result(msg.client_id, &vec![char.id], channel, &mut results);
    }
}
