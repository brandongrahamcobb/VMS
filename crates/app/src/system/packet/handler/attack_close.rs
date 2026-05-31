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

use crate::close_attack::constants::EXP_TABLE;
use crate::component::character::MapleCharacter;
use crate::component::item::MapleItem;
use crate::component::map::InMap;
use crate::message::packet::attack_close::{CloseAttackRequestMessage, CloseAttackResponseMessage, DeadMobMessage};
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{attack_close, codec};
use crate::system::packet::handler::result::HandlerResult;
use action::model::Action;
use bevy::ecs::message::{MessageReader, MessageWriter};
use config::settings;
use inc::helpers;
use ipc::tcp_command::TcpCommand;
use std::collections::HashMap;


pub async fn handle_close_attack_request(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<CloseAttackRequestMessage>,
    command_tx: CustomSender<TcpCommand>,
    mut results: MessageWriter<HandlerResult>,
    chars: Query<&MapleCharacter>,
) -> () {
    for msg in messages.read() {
        let mut actions: Vec<Action> = Vec::new();
        let mut hp_updates: HashMap<MapleMob, i16> = HashMap::new();

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(char) = chars.get(client_entity) else { continue; };

        command_tx.0.send(TcpCommand::CloseAttackRequest { client_id: msg.client_id, char_id: char.id, skill_id: msg.skill_id, mob_damages: msg.mob_damages.clone() });

        for (mob_id, damage) in msg.mob_damages.iter() {
            let Some(mob) = mobs.iter().find(|(_, m, parent)| parent.0 == map && m.id == mob_id);
            let total_damage: i32 = damage.iter().sum();
            mob.hp -= total_damage;
            let hp_percent = (mob.hp * 100 / mob.max_hp) as i16;
            hp_updates.insert(mob, hp_percent);
            if hp_percent == 0 {
                command_tx.0.send(TcpCommand::RandomizedDrops { client_id: msg.client_id, mob_id: mob_id });
            }
        }
        for (mob, hp_percent) in hp_updates {
            let Ok(mob_damage_hp_packet) = mob::builder::build_mob_damage_show_hp_packet(mob.id, hp_percent) else { continue; };
            actions.push(Action::Session(SessionAction::Send {
                packet: mob_damage_hp_packet.finish(),
                scope: SessionScope::Local,
            }));
        }
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions
        });
    }
}

pub async fn handle_dead_mob(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<DeadMobMessage>,
    command_tx: CustomSender<TcpCommand>,
    mut results: MessageWriter<HandlerResult>,
    chars: Query<&MapleCharacter>,
    in_map: Query<&InMap>,
) -> () {
    for msg in messages.read() {
        let mut actions: Vec<Action> = Vec::new();
        let stats_updated: Vec<StatsUpdate> = Vec::new();
        let mode: u8 = 1; // animation 0 fade, 1 drop mob, 2 spawn in
        let owner: i32 = 0; // char id or 0
        let can_pickup: u8 = 0; // 0 everyone 1 owner, 2 party
        let player_drop: bool = false;
        let Ok(meso_rate) = settings::get_meso_drop_rate() else { continue; };

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(char) = chars.get_mut(client_entity) else { continue; };
        let Ok(in_map) = in_maps.get(client_entity) else { continue; };
        let Some(mob) = mobs.iter().find(|(_, m, parent)| parent.0 == in_map.0 && m.id == msg.mob_id);

        let mesos: i32 = helpers::calculate_rand_meso_amount(meso_rate, mob.level);

        let Ok(kill_mob_packet) = mob::builder::build_kill_mob_packet(msg.mob_id) else { continue; };
        actions.push(Action::Broadcast(BroadcastAction::Send {
            packet: kill_mob_packet,
            scope: BroadcastScope::Map,
        }));

        char.exp += mob.exp;
        if char.exp >= EXP_TABLE[char.level as usize] as i32 {
            char.exp = 0;
            char.level += 1;
            stats_update.push(StatsUpdate::Level { level: char.level });
            let Ok(set_level_packet) = codec::player::builder::build_set_level_packet(char.level) else { continue; };
            actions.push(Action::Session(SessionAction::Send {
                packet: set_level_packet.finish(),
                scope: SessionScope::Local,
            }));
            let Ok(set_exp_packet) = codec::player::builder::build_set_exp_packet(0) else { continue; };
            actions.push(Action::Session(SessionAction::Send {
                packet: set_exp_packet.finish(),
                scope: SessionScope::Local,
            }));
            let Ok(level_up_packet) = codec::player::builder::build_level_up_effect_packet(char.id) else { continue; };
            actions.push(Action::Session(SessionAction::Send {
                packet: level_up_packet.finish(),
                scope: SessionScope::Map(MapScope::SameChannelSameWorld),
            }));
 
        } else {
            let Ok(set_exp_packet) = Packet::new_empty().build_set_exp_packet(char.exp)?.finish();
            actions.push(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            }));
        }
        command_tx.0.send(TcpCommand::UpdateStats { client_id: msg.client_id, updates: vec![StatsUpdate::Exp { exp: char.exp }] });

        let drop_from_pos = curr_positions.get(mob.0) else { continue; };
        let drop_from_point: Point = Point {
            x: drop_from_pos.x,
            y: drop_from_pos.y,
        };
        let offset_x = rand::rng().random_range(-50..=50);
        let drop_to_point: Point = Point {
            x: drop_from_pos.x + offset_x,
            y: drop_from_pos.y,
        };
        for item in msg.items {
            let Ok(drop_loot_packet) = item::builder::build_drop_loot_packet(mode,
                    msg.item.id as u32,
                    false,
                    msg.item.wz,
                    owner,
                    can_pickup,
                    drop_to_point.clone(),
                    drop_from_point.clone(),
                    player_drop,
                ) else { continue; };
            actions.push(Action::Broadcast(BroadcastAction::Send {
                packet: drop_loot_packet.finish(),
                scope: BroadcastScope::Map,
            }));
        }
        let Ok(meso_packet) = item::builder::build_drop_loot_packet(
                mode,
                0, // item ID
                true,
                mesos,
                owner,
                can_pickup,
                drop_to_point.clone(),
                drop_from_point.clone(),
                player_drop,
            ) else { continue; };
        actions.push(Action::Broadcast(BroadcastAction::Send {
            packet: meso_packet.finish(),
            scope: BroadcastScope::Map,
        }));
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions
        });
    }
}
 
 

pub async fn handle_close_attack_response(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<CloseAttackResponseMessage>,
    command_tx: CustomSender<TcpCommand>,
    mut results: MessageWriter<HandlerResult>,
    chars: Query<&MapleCharacter>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(char) = chars.get(client_entity) else { continue; };

        let Ok(close_attack_packet) = attack_close::build_close_attack_packet(
                msg.char_id,
                msg.count,
                msg.skill.level,
                msg.skill.wz,
                msg.display,
                msg.toleft,
                msg.stance,
                msg.speed,
                msg.mob_damages.clone(),
            ) else { continue; };
        results.write(HandleResult {
            client_id: msg.client_id,
            actions: vec![Action::Broadcast(BroadcastAction::Send {
                packet: close_attack_packet.finish(),
                scope: BroadcastScope::Map
            })]
        });
    })
}
