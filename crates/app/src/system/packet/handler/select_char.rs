/* select_char/store.rs
 * The purpose of this module is to resolve relevant variables for no-PIC, character selection.
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
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::codec;
use crate::system::system_params::{InParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Res};
use config::settings;
use inc::helpers;

pub fn handle_select_char(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut results: MessageWriter<HandlerResult>,
    mut messages: MessageReader<ReadSelectCharRequestMessage>,
) -> () {
    for msg in messages.read() {
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
        let Some((session_entity, _, _)) = session_params
            .sessions
            .iter()
            .find(|(_, _, parent)| parent.0 == in_session.0)
        else {
            continue;
        };
        let Some((char_entity, char, _)) = session_params
            .chars
            .iter()
            .find(|(_, c, _)| c.id == msg.char_id)
        else {
            continue;
        };
        commands.entity(session_entity).insert(Transitioning {
            map_wz: char.map_wz,
            started_at: Instant::now(),
        });

        commands.entity(client_entity).insert(InChar(char_entity));

        let Ok(mut select_char_packet) =
            codec::login::builder::build_select_char_packet(msg.char_id, octets, channel.port)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: select_char_packet.finish(),
                scope: ActionScope::Local,
            }],
        });
    }
}
