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

use crate::component::character::InChar;
use crate::message::packet::select_char::ReadSelectCharRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::codec;
use crate::system::system_params::{InParams, LocationParams, SessionParams};
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Res};
use config::settings;
use inc::helpers;

pub fn handle_select_char(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    location_params: LocationParams,
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
        let Ok((in_acc_entity, _)) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((acc_entity, _, _)) = session_params.accounts.get(in_acc_entity) else {
            continue;
        };
        let Ok((in_channel_entity, _)) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((_, channel, _)) = location_params.channels.get(in_channel_entity) else {
            continue;
        };
        let Some((char_entity, _, _)) = session_params
            .chars
            .iter()
            .find(|(_, c, parent)| c.id == msg.char_id && parent.0 == acc_entity)
        else {
            continue;
        };

        commands.entity(client_entity).insert(InChar(char_entity));

        let Ok(mut select_char_packet) =
            codec::login::builder::build_select_char_packet(msg.char_id, octets, channel.port)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Break {
                packet: select_char_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}
