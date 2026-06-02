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

use crate::component::channel::{InChannel, MapleChannel};
use crate::component::character::MapleCharacter;
use crate::component::map::{InMap, MapleMap};
use crate::message::packet::take_damage::TakeDamageMessage;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{change_map, codec, take_damage};
use crate::system::packet::handler::result::HandlerResult;
use action::model::{Action, SessionAction};
use action::scope::{MapScope, SessionScope};
use base::character::StatsUpdate;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use core::cmp::Ordering;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_take_damage(
    commands: Commands,
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    chars: Query<&mut MapleCharacter>,
    channels: Query<(Entity, &MapleChannel)>,
    maps: Query<(Entity, &MapleMap, &ChildOf)>,
    in_channels: Query<(Entity, &InChannel)>,
    in_maps: Query<(Entity, &InMap)>,
    mut results: MessageWriter<HandlerResult>,
    mut messages: MessageReader<TakeDamageMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(char) = chars.get_mut(client_entity) else {
            continue;
        };
        let Ok((in_channel_entity, _)) = in_channels.get(client_entity) else {
            continue;
        };
        let Ok((channel_entity, channel)) = channels.get(in_channel_entity) else {
            continue;
        };
        let Ok((in_map_entity, _)) = in_maps.get(client_entity) else {
            continue;
        };
        let Ok((map_entity, map, _)) = maps.get(in_map_entity) else {
            continue;
        };

        let Ok(return_map_wz) = metadata::map::death::get_death_map_by_wz(map.wz) else {
            continue;
        };

        let max_hp = char.max_hp;
        let calc: i16 = char.hp - msg.damage as i16;
        let hp = match calc.cmp(&0) {
            Ordering::Greater | Ordering::Equal => calc,
            _ => 0,
        };
        if hp != 0 {
            char.hp = hp;
            let Ok(take_damage_packet) = take_damage::build_take_damage_packet(hp) else {
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
            let update = StatsUpdate::Health { hp: max_hp };
            command_tx
                .0
                .lock()
                .unwrap()
                .send(AsyncCommand::DatabaseOperation(
                    DatabaseCommand::UpdateStats {
                        client_id: msg.client_id,
                        char_id: char.id,
                        updates: vec![update],
                    },
                ))
                .unwrap();
            commands.entity(client_entity).remove::<InMap>();
            let Some((map_entity, _, _)) = maps
                .iter()
                .find(|(_, m, parent)| m.wz == return_map_wz && parent.0 == channel_entity)
            else {
                continue;
            };
            commands.entity(client_entity).insert(InMap(map_entity));
            let Ok(mut despawn_packet) =
                codec::player::builder::build_despawn_player_packet(char.id)
            else {
                continue;
            };
            let Ok(mut set_field_packet) =
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
    }
}
