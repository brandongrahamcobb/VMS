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

use crate::component::character::{InChar, MapleCharacter};
use crate::component::item::MapleItem;
use crate::component::mob::MapleMob;
use crate::message::packet::attack_close::{
    CloseAttackResponseMessage, DeadMobResponseMessage, ReadCloseAttackRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{attack_close, codec};
use crate::system::packet::handler::constants::EXP_TABLE;
use crate::system::system_params::{
    InParams, LocationParams, PositionParams, SessionParams, StatParams,
};
use action::model::{Action, BroadcastAction, SessionAction};
use action::scope::{BroadcastScope, MapScope, SessionScope};
use base::character::StatsUpdate;
use base::map::Point;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use config::settings;
use inc::helpers;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;
use rand::RngExt;
use std::collections::HashMap;

pub fn handle_close_attack_request(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    location_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut stat_params: StatParams,
    mobs: Query<(Entity, &MapleMob, &ChildOf)>,
    mut messages: MessageReader<ReadCloseAttackRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let mut actions: Vec<Action> = Vec::new();
        let mut hp_updates: HashMap<u32, i16> = HashMap::new();

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_map_entity, _)) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Ok((map_entity, _, _)) = location_params.maps.get(in_map_entity) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char_entity) else {
            continue;
        };

        command_tx
            .0
            .lock()
            .unwrap()
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
            ));

        for (mob_id, damage) in msg.mob_damages.iter() {
            let Some((mob_entity, mob, _)) = mobs
                .iter()
                .find(|(_, m, parent)| parent.0 == map_entity && m.id == *mob_id)
            else {
                continue;
            };
            let Some((_, mut health, _)) = stat_params
                .healths
                .iter_mut()
                .find(|(_, _, parent)| parent.0 == mob_entity)
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
                    .lock()
                    .unwrap()
                    .send(AsyncCommand::DatabaseOperation(
                        DatabaseCommand::DeadMobRequest {
                            client_id: msg.client_id,
                            mob_id: *mob_id,
                        },
                    ));
            }
        }
        for (mob_id, hp_percent) in hp_updates {
            let Ok(mut mob_damage_hp_packet) =
                codec::mob::builder::build_mob_damage_show_hp_packet(mob_id, hp_percent)
            else {
                continue;
            };
            actions.push(Action::Session(SessionAction::Send {
                packet: mob_damage_hp_packet.finish(),
                scope: SessionScope::Local,
            }));
        }
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions,
        });
    }
}

pub fn handle_dead_mob(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    location_params: LocationParams,
    in_params: InParams,
    mut session_params: SessionParams,
    mut stat_params: StatParams,
    position_params: PositionParams,
    mobs: Query<(Entity, &MapleMob, &ChildOf)>,
    items: Query<&MapleItem>,
    mut messages: MessageReader<DeadMobResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let mut actions: Vec<Action> = Vec::new();
        let mut stats_updates: Vec<StatsUpdate> = Vec::new();
        let mode: u8 = 1; // animation 0 fade, 1 drop mob, 2 spawn in
        let owner: i32 = 0; // char id or 0
        let can_pickup: u8 = 0; // 0 everyone 1 owner, 2 party
        let player_drop: bool = false;
        let Ok(meso_rate) = settings::get_meso_drop_rate() else {
            continue;
        };

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, mut char, _)) = session_params.chars.get_mut(in_char_entity) else {
            continue;
        };
        let Ok((in_map_entity, _)) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Ok((map_entity, _, _)) = location_params.maps.get(in_map_entity) else {
            continue;
        };
        let Some((mob_entity, mob, _)) = mobs
            .iter()
            .find(|(_, m, parent)| parent.0 == map_entity && m.id == msg.mob_id)
        else {
            continue;
        };
        let Ok((_, mut exp, _)) = stat_params.exps.get_mut(mob_entity) else {
            continue;
        };

        let mesos: i32 = helpers::calculate_rand_meso_amount(meso_rate, mob.base.level);

        let Ok(kill_mob_packet) = codec::mob::builder::build_kill_mob_packet(msg.mob_id) else {
            continue;
        };
        actions.push(Action::Broadcast(BroadcastAction::Send {
            packet: kill_mob_packet,
            scope: BroadcastScope::Map,
        }));

        char.exp += exp.amount;
        if char.exp >= EXP_TABLE[char.level as usize] as i32 {
            exp.amount = 0;
            char.level += 1;
            stats_updates.push(StatsUpdate::Level { level: char.level });
            let Ok(mut set_level_packet) =
                codec::player::builder::build_set_level_packet(char.level)
            else {
                continue;
            };
            actions.push(Action::Session(SessionAction::Send {
                packet: set_level_packet.finish(),
                scope: SessionScope::Local,
            }));
            let Ok(mut set_exp_packet) = codec::player::builder::build_set_exp_packet(0) else {
                continue;
            };
            actions.push(Action::Session(SessionAction::Send {
                packet: set_exp_packet.finish(),
                scope: SessionScope::Local,
            }));
            let Ok(mut level_up_packet) =
                codec::player::builder::build_level_up_effect_packet(char.id)
            else {
                continue;
            };
            actions.push(Action::Session(SessionAction::Send {
                packet: level_up_packet.finish(),
                scope: SessionScope::Map(MapScope::SameChannelSameWorld),
            }));
        } else {
            let Ok(mut set_exp_packet) = codec::player::builder::build_set_exp_packet(char.exp)
            else {
                continue;
            };
            actions.push(Action::Session(SessionAction::Send {
                packet: set_exp_packet.finish(),
                scope: SessionScope::Local,
            }));
        }
        command_tx
            .0
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::UpdateStats {
                    client_id: msg.client_id,
                    char_id: char.id,
                    updates: vec![StatsUpdate::Exp { exp: char.exp }],
                },
            ));

        let Ok((_, drop_from_pos, _)) = position_params.curr_positions.get(mob_entity) else {
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
        for (base_item, item_model) in msg.items.clone() {
            let item_enitity = commands
                .spawn((
                    MapleItem::from((base_item.clone(), item_model)),
                    ChildOf(map_entity),
                ))
                .id();
            let Ok(item) = items.get(item_enitity) else {
                continue;
            };
            let Ok(mut drop_loot_packet) = codec::item::builder::build_drop_loot_packet(
                mode,
                item.id as u32,
                false,
                base_item.wz,
                owner,
                can_pickup,
                drop_to_point.clone(),
                drop_from_point.clone(),
                player_drop,
            ) else {
                continue;
            };
            actions.push(Action::Broadcast(BroadcastAction::Send {
                packet: drop_loot_packet.finish(),
                scope: BroadcastScope::Map,
            }));
        }
        let Ok(mut meso_packet) = codec::item::builder::build_drop_loot_packet(
            mode,
            0, // item ID
            true,
            mesos,
            owner,
            can_pickup,
            drop_to_point.clone(),
            drop_from_point.clone(),
            player_drop,
        ) else {
            continue;
        };
        actions.push(Action::Broadcast(BroadcastAction::Send {
            packet: meso_packet.finish(),
            scope: BroadcastScope::Map,
        }));
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions,
        });
    }
}

pub fn handle_close_attack_response(
    client_map: Res<ClientMap>,
    chars: Query<&MapleCharacter>,
    in_chars: Query<(Entity, &InChar)>,
    mut messages: MessageReader<CloseAttackResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_chars.get(client_entity) else {
            continue;
        };
        let Ok(char) = chars.get(in_char_entity) else {
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
            actions: vec![Action::Broadcast(BroadcastAction::Send {
                packet: close_attack_packet.finish(),
                scope: BroadcastScope::Map,
            })],
        });
    }
}
