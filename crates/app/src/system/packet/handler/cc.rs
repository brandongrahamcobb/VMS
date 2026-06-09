/* cc/store.rs
 * The purpose of this module is to resolve relevant variables for changing channels.
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

use crate::component::channel::InChannel;
use crate::message::packet::cc::ReadChangeChannelRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::{cc, codec};
use crate::system::system_params::{InParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::{ActionScope, MapScope};
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Commands;
use bevy::ecs::system::Res;
use config::settings;
use inc::helpers;

pub fn handle_change_channel(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadChangeChannelRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Ok(addr) = settings::get_routing_address() else {
            continue;
        };
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_world) = in_params.in_worlds.get(client_entity) else {
            continue;
        };
        let Some((channel_entity, channel, _)) = loc_params
            .channels
            .iter()
            .find(|(_, c, parent)| c.id == msg.channel_id && parent.0 == in_world.0)
        else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };

        commands.entity(client_entity).remove::<InChannel>();
        commands
            .entity(client_entity)
            .insert(InChannel(channel_entity));

        let Ok(mut despawn_packet) = codec::player::builder::build_despawn_player_packet(char.id)
        else {
            continue;
        };
        let Ok(mut cc_packet) = cc::build_channel_change_packet(octets, channel.port) else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![
                Action::HandlerAction {
                    packet: despawn_packet.finish(),
                    scope: ActionScope::Map(MapScope::SameChannelSameWorld),
                },
                Action::HandlerAction {
                    packet: cc_packet.finish(),
                    scope: ActionScope::Local,
                }, // break transition TODO
            ],
        });
    }
}
