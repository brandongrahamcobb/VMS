/* mob_respawn/handler.rs
 * The purpose of this module is to handle mob respawning per map.
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

use crate::manager::{MS_PER_TICK, TickManager};
use crate::mob_respawn::error::MobRespawnError;
use crate::mob_respawn::store::MobRespawnStore;
use action::event::TickEvent;
use action::model::{Action, BroadcastAction};
use action::scope::BroadcastScope;
use core::time::Duration;
use entity::map::model::Point;
use packet::model::Packet;
use state::model::SharedState;
use std::sync::Arc;
use tokio::time::Instant;

pub struct MobRespawnTick;

impl Default for MobRespawnTick {
    fn default() -> Self {
        Self::new()
    }
}

impl MobRespawnTick {
    pub fn new() -> Self {
        Self
    }

    pub async fn spawn(&self, state: &SharedState) -> Result<(), MobRespawnError> {
        let tick_tx_clone = {
            let state = state.lock().await;
            state.tick_tx.clone()
        };
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
                let now: Instant = Instant::now();
                let maps: Vec<(i16, u8, i32)> = {
                    let state = state_clone.lock().await;
                    let worlds = state.worlds.read().await;
                    worlds
                        .iter()
                        .flat_map(|(world_id, world)| {
                            world.channels.iter().flat_map(|(channel_id, channel)| {
                                channel
                                    .maps
                                    .keys()
                                    .map(|map_wz| (*world_id, *channel_id, *map_wz))
                            })
                        })
                        .collect()
                };
                for (world_id, channel_id, map_wz) in maps {
                    let store = match MobRespawnStore::store_mob_respawn(
                        &state_clone,
                        now,
                        world_id,
                        channel_id,
                        map_wz,
                    )
                    .await
                    {
                        Ok(s) => s,
                        Err(_) => continue,
                    };
                    let event = match MobRespawnTick::build_respawn_event(&store).await {
                        Ok(e) => e,
                        Err(_) => continue,
                    };
                    let _ = tick_tx_clone.send(event);
                }
            }
        });
        Ok(())
    }

    pub async fn build_respawn_event(
        store: &MobRespawnStore,
    ) -> Result<TickEvent, MobRespawnError> {
        let mut event: TickEvent = TickEvent::new();
        for (mob_id, mob_life) in store.to_respawn.clone() {
            let packet = Packet::new_empty()
                .build_spawn_mob_packet(mob_id, &mob_life)?
                .finish();
            event.add_action(Action::Broadcast(BroadcastAction::Send {
                packet: packet.clone(),
                scope: BroadcastScope::Map {
                    world_id: store.world_id,
                    channel_id: store.channel_id,
                    map_wz: store.map_wz,
                },
            }));
            let packet = Packet::new_empty()
                .build_spawn_mob_controller_packet(
                    mob_id,
                    store.mode,
                    mob_life.wz,
                    store.stance,
                    mob_life.fh,
                    store.effect,
                    &Point {
                        x: mob_life.x,
                        y: mob_life.y,
                    },
                    store.team,
                )?
                .finish();
            event.add_action(Action::Broadcast(BroadcastAction::Send {
                packet: packet.clone(),
                scope: BroadcastScope::MapChar {
                    world_id: store.world_id,
                    channel_id: store.channel_id,
                    map_wz: store.map_wz,
                },
            }));
        }
        Ok(event)
    }
}
