/* close_attack/store.rs
 * The purpose of this module is to resolve relevant variables for close attacks.
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
use crate::component::character::MapleCharacter;
use crate::component::map::MapleMap;
use crate::component::session::MapleSession;
use crate::message::packet::take_damage::TakeDamageMessage;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::{change_map, codec, take_damage};
use crate::system::packet::handler::result::HandlerResult;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Query;
use core::cmp::Ordering;
use ipc::tcp_command::TcpCommand;

pub fn handle_take_damage(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<TakeDamageMessage>,
    mut results: MessageWriter<HandlerResult>,
    mut sessions: Query<&mut MapleSession>,
    chars: Query<(&mut MapleCharacter, &ChildOf)>,
    channels: Query<(&MapleChannel, &ChildOf)>,
    maps: Query<(&MapleMap, &ChildOf)>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Some(session) = sessions.get_mut(client_entity) else {
            continue;
        };
        let Some(char) = chars.get_mut(client_entity) else {
            continue;
        };
        let Some(channel) = channels.get(client_entity) else {
            continue;
        };
        let Some(map) = map.get(client_entity) else {
            continue;
        };

        let return_map_wz: i32 = metadata::map::death::get_death_map_by_wz(map_wz)?;

        let max_hp = char.max_hp;
        let calc: i16 = char.hp - msg.damage as i16;
        let hp = match calc.cmp(&0) {
            Ordering::Greater | Ordering::Equal => calc,
            _ => 0,
        };
        if hp != 0 {
            char.hp = hp;
            let Some(take_damage_packet): Option<Packet> =
                take_damage::build_take_damage_packet(hp)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::Session(SessionAction::Send {
                    packet: take_damage_packet.finish(),
                    scope: SessionScope::Local,
                })],
            });
        } else {
            char.hp = max_hp;
            session.map_wz = return_map_wz;
            let Some(despawn_packet): Option<Packet> =
                codec::player::builder::build_despawn_player_packet(char.id)
            else {
                continue;
            };
            let Some(set_field_packet): Option<Packet> =
                change_map::build_set_field_change_map_packet(channel.id, return_map_wz, 0)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![
                    Action::Session(SessionAction::Send {
                        packet: despawn_packet.finish(),
                        scope: SessionScope::Map(MapScope::SameChannelSameWorld),
                    }),
                    Action::Session(SessionAction::Send {
                        packet: set_field_packet.clone(),
                        scope: SessionScope::Local,
                    }),
                ],
            });
        };
        command_tx
            .0
            .send(TcpCommand::UpdateHealth {
                client_id: msg.client_id,
                char_id: char.id,
                hp,
            })
            .unwrap();
    }
}
