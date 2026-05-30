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

use crate::component::channel::MapleChannel;
use crate::component::session::MapleSession;
use crate::component::world::MapleWorld;
use crate::message::packet::select_char_with_pic::SelectCharWithPicResponseMessage;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::spw;
use crate::system::packet::handler::result::HandlerResult;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};
use config::settings;
use inc::helpers;
use ipc::tcp_command::TcpCommand;

pub async fn handle_select_char_with_pic_request(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<SelectCharWithPicRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
    sessions: Query<&mut MapleSession>,
    worlds: Query<(Entity, &MapleWorld)>,
    channels: Query<(&MapleChannel, &ChildOf)>,
) -> () {
    for msg in messages.read() {
        let Some(client_entity): Option<Entity> = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Some(session): Option<MapleSession> = sessions.get_mut(client_entity) else {
            continue;
        };
        let Some(acc_id): Option<i32> = session.acc_id else {
            continue;
        };

        command_tx
            .0
            .send(TcpCommand::SelectCharWithPicCommand((msg, acc_id).into()))
            .unwrap()
    }
}

pub async fn handle_select_char_with_pic_response(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<SelectCharWithPicResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
    sessions: Query<&mut MapleSession>,
    worlds: Query<(Entity, &MapleWorld)>,
    channels: Query<(&MapleChannel, &ChildOf)>,
) -> () {
    for msg in messages.read() {
        if msg.status {
            let Some(client_entity): Option<&Entity> = client_map.0.get(&msg.client_id) else {
                continue;
            };
            let Some((char_entity, character, _)) = chars
                .iter()
                .find(|(_, c, parent)| c.id == msg.char.id && parent.0 == client_entity)
            else {
                continue;
            };
            commands.insert(client_entity).insert(InChar(char_entity));
            let Some((world_entity, world)) =
                worlds.iter().find(|(_, w)| w.id == msg.char.world_id)
            else {
                continue;
            };
            commands.insert(client_entity).insert(InWorld(world_entity));
            let Some((channel_entity, channel, _)) = channels
                .iter()
                .find(|(_, c, parent)| c.id == msg.channel_id && parent.0 == world_entity.0)
            else {
                continue;
            };
            commands
                .insert(client_entity)
                .insert(InChannel(channel_entity));
            let Some((map_entity, map, _)) = maps
                .iter()
                .find(|(_, m, parent)| m.id == msg.char.map_wz && parent.0 == channel_entity.0)
            else {
                continue;
            };

            let Ok(acc) = accounts.get(client_entity) else {
                continue;
            };

            let Some(port): Option<i16> =
                domain::channel::find_channel_port(&worlds, &channels, world.id, channel.id);

            let Ok(addr) = settings::get_routing_address() else {
                continue;
            };
            let octets: [u8; 4] = helpers::convert_to_ip_array(addr);

            let Ok(select_char_packet) =
                codec::login::builder.build_select_char_packet(msg.char.id, octets, port)
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
        } else {
            let success_status: bool = false;
            let Ok(select_char_failed_packet) = spw::build_spw_packet(success_status) else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::Session(SessionAction::Send {
                    packet: select_char_failed_packet.finish(),
                    scope: SessionScope::Local,
                })],
            });
        }
    }
}
