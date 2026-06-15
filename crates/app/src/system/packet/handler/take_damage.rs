/* app/src/system/packet/handler/take_damage.rs
 * The purpose of this module is to handle player damage taken system messages.
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

use crate::component::map::InMap;
use crate::message::packet::take_damage::ReadTakeDamageRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{change_map, codec, take_damage};
use crate::system::system_params::{InParams, LocationParams, SessionParams, StatParams};
use action::model::Action;
use action::scope::{ActionScope, MapScope};
use base::character::StatsUpdate;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Res};
use core::cmp::Ordering;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_take_damage(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut stats_params: StatParams,
    mut results: MessageWriter<HandlerResult>,
    mut messages: MessageReader<ReadTakeDamageRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((_, channel, _)) = loc_params.channels.get(in_channel.0) else {
            continue;
        };
        let Ok(in_map) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Ok((_, map, _)) = loc_params.maps.get(in_map.0) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };
        let Some((mut hp, _)) = stats_params
            .healths
            .iter_mut()
            .find(|(_, parent)| parent.0 == in_char.0)
        else {
            continue;
        };

        let Ok(return_map_wz) = metadata::map::death::get_death_map_by_wz(map.base.wz) else {
            continue;
        };

        let calc: i32 = hp.amount - msg.damage;
        let recalculated_hp = match calc.cmp(&0) {
            Ordering::Greater | Ordering::Equal => calc,
            _ => 0,
        };
        if recalculated_hp != 0 {
            hp.amount = recalculated_hp;
            let Ok(mut take_damage_packet) = take_damage::build_take_damage_packet(recalculated_hp)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: take_damage_packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        } else {
            hp.amount = hp.max;
            let update = StatsUpdate::Health { hp: hp.max };
            command_tx
                .0
                .send(AsyncCommand::DatabaseOperation(
                    DatabaseCommand::UpdateStats {
                        client_id: msg.client_id,
                        char_id: char.id,
                        updates: vec![update],
                    },
                ))
                .unwrap();
            commands.entity(client_entity).remove::<InMap>();
            let Some((map_entity, _, _)) = loc_params
                .maps
                .iter()
                .find(|(_, m, parent)| parent.0 == in_channel.0 && m.base.wz == return_map_wz)
            else {
                continue;
            };
            commands.entity(client_entity).insert(InMap(map_entity));
            let Ok(mut despawn_packet) = codec::player::spawn::build_despawn_player_packet(char.id)
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
                    Action::HandlerAction {
                        packet: despawn_packet.finish(),
                        scope: ActionScope::Map(MapScope::SameChannelSameWorld),
                    },
                    Action::HandlerAction {
                        packet: set_field_packet.finish(),
                        scope: ActionScope::Local,
                    },
                ],
            });
        };
    }
}
