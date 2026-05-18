/* map/respawn.rs
 * The purpose of this module is to provide a respawn mechanism for mob-containing maps.
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

use crate::models::map::error::MapError;
use crate::models::mob::wrapper::Mob;
use crate::net::packet::model::Packet;
use crate::runtime::relay::execute;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::state::SharedState;
use crate::runtime::tick::{MS_PER_TICK, TickManager};
use core::time::Duration;
use std::sync::Arc;
use tokio::time::Instant;

pub async fn respawn_tick(
    state: &SharedState,
    world_id: i16,
    channel_id: u8,
    map_wz: i32,
) -> Result<(), MapError> {
    let tick_manager = Arc::new(TickManager::new());
    let tick_manager_clone = Arc::clone(&tick_manager);
    let state_clone = Arc::clone(state);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(MS_PER_TICK as u64));
        loop {
            tokio::select! {
                _ = interval.tick() => {}
                _ = tick_manager_clone.wait() => {}
            }
            tick_manager_clone.tick();
            let now = Instant::now();
            let respawned: Vec<(u32, Mob)> = {
                let state = state_clone.lock().await;
                state
                    .with_mut_map(
                        world_id,
                        channel_id,
                        map_wz,
                        |map| -> Result<Vec<(u32, Mob)>, MapError> {
                            let due: Vec<u32> = map
                                .dead_mobs
                                .iter()
                                .filter(|(_, dead)| {
                                    now.duration_since(dead.died_at) >= dead.respawn_time
                                })
                                .map(|(mob_id, _)| *mob_id)
                                .collect();
                            let mut respawned = Vec::new();
                            for mob_id in due {
                                if let Some(mut dead) = map.dead_mobs.remove(&mob_id) {
                                    dead.model.hp = dead.model.max_hp;
                                    let map_mob = Mob {
                                        model: dead.model.clone(),
                                    };
                                    let respawned_mob = Mob {
                                        model: dead.model.clone(),
                                    };
                                    map.mobs.insert(mob_id, map_mob);
                                    respawned.push((mob_id, respawned_mob));
                                }
                            }
                            Ok(respawned)
                        },
                    )
                    .await
                    .unwrap_or_else(|_| Ok(Vec::new()))
                    .unwrap_or_else(|_| Vec::new())
            };
            for (mob_id, mob) in respawned {
                if let Ok(packet) = Packet::new_empty()
                    .build_spawn_mob_packet(&mob)
                    .map(|p| p.finish())
                {
                    let _ = execute::manager::broadcast(
                        &state_clone,
                        &packet,
                        Some(world_id),
                        Some(channel_id),
                        Some(map_wz),
                        &Scope::Map(MapScope::SameChannelSameWorld),
                    )
                    .await;
                }
                if let Ok(packet) = Packet::new_empty()
                    .build_mob_move_packet(
                        mob_id as u32,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0, // skills all 0
                        mob.model.pos_x,
                        mob.model.pos_y,
                        0, // command
                        mob.model.pos_x,
                        mob.model.pos_y,
                        mob.model.pos_x,
                        mob.model.pos_y,
                        mob.model.fh,
                        0, // stance
                        0, // duration
                    )
                    .map(|p| p.finish())
                {
                    let _ = execute::manager::broadcast(
                        &state_clone,
                        &packet,
                        Some(world_id),
                        Some(channel_id),
                        Some(map_wz),
                        &Scope::Map(MapScope::SameChannelSameWorld),
                    )
                    .await;
                }
            }
        }
    });
    Ok(())
}
