/* close_attack/handler.rs
 * The purpose of this module is to handle close attacks.
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

use crate::models::map::model::Point;
use crate::net::action::{Action, BroadcastAction, SessionAction};
use crate::net::packet::handler::close_attack::error::CloseAttackError;
use crate::net::packet::handler::close_attack::reader::CloseAttackReader;
use crate::net::packet::handler::close_attack::store::CloseAttackStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{BroadcastScope, MapScope, SessionScope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use rand::RngExt;

pub struct CloseAttackHandler;

impl CloseAttackHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, CloseAttackError> {
        let reader: CloseAttackReader = CloseAttackReader::read_close_attack_packet(packet)?;
        let store: CloseAttackStore =
            CloseAttackStore::store_close_attack(state, session, &reader).await?;
        let result = self.build_close_attack_result(&store).await?;
        Ok(result)
    }

    async fn build_close_attack_result(
        &self,
        store: &CloseAttackStore,
    ) -> Result<HandlerResult, CloseAttackError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet = Packet::new_empty()
            .build_close_attack_packet(
                store.char_id,
                store.count,
                store.skill_level,
                store.skill_wz,
                store.display,
                store.toleft,
                store.stance,
                store.speed,
                store.mob_damages.clone(),
            )?
            .finish();
        result.add_action(Action::Broadcast(BroadcastAction::Send {
            packet: packet.clone(),
            scope: BroadcastScope::Map {
                world_id: store.world_id,
                channel_id: store.channel_id,
                map_wz: store.map_wz,
            },
        }));
        for (mob_id, hp_percent) in store.hp_updates.clone() {
            let packet = Packet::new_empty()
                .build_mob_damage_show_hp_packet(mob_id, hp_percent)?
                .finish();
            result.add_action(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            }));
        }
        for (mob_id, mob) in store.dead_mobs.iter() {
            let packet = Packet::new_empty().build_kill_mob_packet(*mob_id)?.finish();
            result.add_action(Action::Broadcast(BroadcastAction::Send {
                packet: packet.clone(),
                scope: BroadcastScope::Map {
                    world_id: store.world_id,
                    channel_id: store.channel_id,
                    map_wz: store.map_wz,
                },
            }));
            let drop_from: Point = Point {
                x: mob.model.pos_x,
                y: mob.model.pos_y,
            };
            let offset_x = rand::rng().random_range(-50..=50);
            let drop_to: Point = Point {
                x: mob.model.pos_x + offset_x,
                y: mob.model.pos_y,
            };
            if let Some(drops) = store.dead_mobs_drops.get(&mob_id) {
                for (count, drop) in drops {
                    let item_id: i32 = drop.model.get_id()?;
                    let item_wz: i32 = drop.model.wz;
                    for _ in 0..*count {
                        let packet = Packet::new_empty()
                            .build_drop_loot_packet(
                                store.mode,
                                item_id as u32,
                                false,
                                item_wz,
                                store.owner,
                                store.can_pickup,
                                drop_to.clone(),
                                drop_from.clone(),
                                store.player_drop,
                            )?
                            .finish();
                        result.add_action(Action::Broadcast(BroadcastAction::Send {
                            packet: packet.clone(),
                            scope: BroadcastScope::Map {
                                world_id: store.world_id,
                                channel_id: store.channel_id,
                                map_wz: store.map_wz,
                            },
                        }));
                    }
                }
            }

            let offset_x = rand::rng().random_range(-50..=50);
            let drop_to: Point = Point {
                x: mob.model.pos_x + offset_x,
                y: mob.model.pos_y,
            };
            if let Some(mesos) = store.dead_mobs_mesos.get(&mob_id) {
                let packet = Packet::new_empty()
                    .build_drop_loot_packet(
                        store.mode,
                        0, // item ID
                        true,
                        *mesos,
                        store.owner,
                        store.can_pickup,
                        drop_to,
                        drop_from,
                        store.player_drop,
                    )?
                    .finish();
                result.add_action(Action::Broadcast(BroadcastAction::Send {
                    packet: packet.clone(),
                    scope: BroadcastScope::Map {
                        world_id: store.world_id,
                        channel_id: store.channel_id,
                        map_wz: store.map_wz,
                    },
                }));
            }
            if let Some(mob) = store.dead_mobs.get(&mob_id) {
                if !store.levelup {
                    let exp = store.base_exp + mob.info.exp;
                    let packet = Packet::new_empty().build_set_exp_packet(exp)?.finish();
                    result.add_action(Action::Session(SessionAction::Send {
                        packet: packet.clone(),
                        scope: SessionScope::Local,
                    }));
                } else {
                    let packet = Packet::new_empty()
                        .build_set_level_packet(store.level)?
                        .finish();
                    result.add_action(Action::Session(SessionAction::Send {
                        packet: packet.clone(),
                        scope: SessionScope::Local,
                    }));
                    let exp = 0;
                    let packet = Packet::new_empty().build_set_exp_packet(exp)?.finish();
                    result.add_action(Action::Session(SessionAction::Send {
                        packet: packet.clone(),
                        scope: SessionScope::Local,
                    }));
                    let packet = Packet::new_empty()
                        .build_level_up_effect_packet(store.char_id)?
                        .finish();
                    result.add_action(Action::Session(SessionAction::Send {
                        packet: packet.clone(),
                        scope: SessionScope::Map(MapScope::SameChannelSameWorld),
                    }));
                }
            }
        }
        Ok(result)
    }
}
