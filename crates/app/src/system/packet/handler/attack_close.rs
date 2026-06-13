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

use crate::component::item::MapleItem;
use crate::component::mob::MapleMob;
use crate::message::packet::attack_close::{
    CloseAttackResponseMessage, DeadMobResponseMessage, ReadCloseAttackRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::attack_close;
use crate::system::packet::handler::constants::EXP_TABLE;
use crate::system::packet::handler::result::{
    drop_items_and_mesos_result, kill_mob_result, level_up_result, mob_damage_result,
    set_exp_result, spawn_mob_controller_result,
};
use crate::system::system_params::{
    InParams, LocationParams, PositionParams, SessionParams, StatParams,
};
use action::model::Action;
use action::scope::{ActionScope, MapScope};
use base::character::StatsUpdate;
use base::map::Point;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;
use rand::RngExt;
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

        for (mob_id, damage) in msg.mob_damages.iter() {
            let Some((mob_entity, mob, _)) = mobs
                .iter()
                .find(|(_, m, parent)| parent.0 == map_entity && m.id == *mob_id)
            else {
                continue;
            };
            spawn_mob_controller_result::write_result(msg.client_id, &vec![*mob], &mut results);
            let Some((mut health, _)) = stat_params
                .healths
                .iter_mut()
                .find(|(_, parent)| parent.0 == mob_entity)
            else {
                continue;
            };
            let total_damage: i32 = damage.iter().sum();
            health.amount -= total_damage;
            let hp_percent = (health.amount * 100 / mob.base.max_hp) as i16;
            hp_updates.insert(mob.id, hp_percent);
            if hp_percent == 0 {
                command_tx
                    .0
                    .send(AsyncCommand::DatabaseOperation(
                        DatabaseCommand::DeadMobRequest {
                            client_id: msg.client_id,
                            mob_id: *mob_id,
                        },
                    ))
                    .unwrap();
            }
        }
        mob_damage_result::write_result(msg.client_id, &hp_updates, &mut results);
    }
}

pub fn handle_dead_mob(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    mut session_params: SessionParams,
    mut stat_params: StatParams,
    pos_params: PositionParams,
    mobs: Query<(Entity, &MapleMob, &ChildOf)>,
    items: Query<&MapleItem>,
    mut messages: MessageReader<DeadMobResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let mut stats_updates: Vec<StatsUpdate> = Vec::new();

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, mut char, _)) = session_params.chars.get_mut(in_char.0) else {
            continue;
        };
        let Ok(in_map) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Some((mob_entity, mob, _)) = mobs
            .iter()
            .find(|(_, m, parent)| parent.0 == in_map.0 && m.id == msg.mob_id)
        else {
            continue;
        };
        kill_mob_result::write_result(msg.client_id, &vec![*mob], &mut results);
        let Some((mut exp, _)) = stat_params
            .exps
            .iter_mut()
            .find(|(_, parent)| parent.0 == mob_entity)
        else {
            continue;
        };

        char.exp += exp.amount;
        if char.exp >= EXP_TABLE[char.level as usize] as i32 {
            exp.amount = 0;
            char.level += 1;
            stats_updates.push(StatsUpdate::Level { level: char.level });
            level_up_result::write_result(msg.client_id, &vec![char.clone()], &mut results);
        } else {
            set_exp_result::write_result(msg.client_id, &vec![char.clone()], &mut results);
        }
        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::UpdateStats {
                    client_id: msg.client_id,
                    char_id: char.id,
                    updates: vec![StatsUpdate::Exp { exp: char.exp }],
                },
            ))
            .unwrap();

        let Ok((drop_from_pos, _)) = pos_params.curr_positions.get(mob_entity) else {
            continue;
        };
        let drop_from_point: Point = Point {
            x: drop_from_pos.x,
            y: drop_from_pos.y,
        };
        let offset_x = rand::rng().random_range(-50..=50);
        let drop_to_point: Point = Point {
            x: drop_from_pos.x + offset_x,
            y: drop_from_pos.y,
        };
        let mut item_vec: Vec<MapleItem> = Vec::new();
        for (base_item, item_model) in msg.items.clone() {
            let item_enitity = commands
                .spawn((
                    MapleItem::from((base_item.clone(), item_model)),
                    ChildOf(in_map.0),
                ))
                .id();
            let Ok(item) = items.get(item_enitity) else {
                continue;
            };
            item_vec.push(item.clone());
        }
        drop_items_and_mesos_result::write_result(
            msg.client_id,
            &vec![*mob],
            &item_vec,
            drop_to_point,
            drop_from_point,
            &mut results,
        );
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
