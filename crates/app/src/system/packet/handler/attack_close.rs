/* app/src/system/handler/attack_close.rs
 * The purpose of this module is to handle close attack system messages.
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

use crate::component::mob::MapleMob;
use crate::message::packet::attack_close::{
    CloseAttackResponseMessage, ReadCloseAttackRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::attack_close;
use crate::system::packet::handler::result::{mob_damage_result, spawn_mob_controller_result};
use crate::system::system_params::{InParams, LocationParams, SessionParams, StatParams};
use action::model::Action;
use action::scope::{ActionScope, MapScope};
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;
use std::collections::HashMap;

pub fn handle_close_attack_request(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut stat_params: StatParams,
    mobs: Query<(Entity, &MapleMob, &ChildOf)>,
    mut messages: MessageReader<ReadCloseAttackRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let mut hp_updates: HashMap<u32, i16> = HashMap::new();

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_map) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Ok((map_entity, _, _)) = loc_params.maps.get(in_map.0) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };

        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::CloseAttackRequest {
                    client_id: msg.client_id,
                    char_id: char.id,
                    count: msg.count,
                    skill_id: msg.skill_id,
                    display: msg.display,
                    toleft: msg.toleft,
                    stance: msg.stance,
                    speed: msg.speed,
                    mob_damages: msg.mob_damages.clone(),
                },
            ))
            .unwrap();

        let Some(mobs_vec): Option<Vec<MapleMob>> = msg
            .mob_damages
            .iter()
            .map(|(mid, _)| {
                mobs.iter()
                    .find(|(_, m, parent)| parent.0 == map_entity && m.id == *mid)
                    .map(|(_, m, _)| *m)
            })
            .collect()
        else {
            continue;
        };
        spawn_mob_controller_result::write_result(msg.client_id, &mobs_vec, &mut results);

        for (mob_id, damage) in msg.mob_damages.iter() {
            let Some((mob_entity, mob, _)) = mobs
                .iter()
                .find(|(_, m, parent)| parent.0 == map_entity && m.id == *mob_id)
            else {
                continue;
            };
            let Some((mut hp, _)) = stat_params
                .healths
                .iter_mut()
                .find(|(_, parent)| parent.0 == mob_entity)
            else {
                continue;
            };
            let total_damage: i32 = damage.iter().sum();
            hp.amount -= total_damage;
            let hp_percent = (hp.amount * 100 / mob.base.max_hp as i32) as i16;
            hp_updates.insert(mob.id, hp_percent);
            if hp_percent == 0 {
                command_tx
                    .0
                    .send(AsyncCommand::DatabaseOperation(
                        DatabaseCommand::DeadMobRequest {
                            client_id: msg.client_id,
                            mob_id: *mob_id,
                            mob_wz: mob.base.wz,
                        },
                    ))
                    .unwrap();
            }
        }
        mob_damage_result::write_result(msg.client_id, &hp_updates, &mut results);
    }
}

pub fn handle_close_attack_response(
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<CloseAttackResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };

        let Ok(mut close_attack_packet) = attack_close::build_close_attack_packet(
            char.id,
            msg.count,
            msg.skill_model.level,
            msg.base_skill.wz,
            msg.display,
            msg.toleft,
            msg.stance,
            msg.speed,
            msg.mob_damages.clone(),
        ) else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: close_attack_packet.finish(),
                scope: ActionScope::Map(MapScope::SameChannelSameWorld),
            }],
        });
    }
}
