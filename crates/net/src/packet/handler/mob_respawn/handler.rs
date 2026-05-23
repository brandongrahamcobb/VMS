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

use entity::map::model::Point;
use crate::action::model::{Action, BroadcastAction};
use crate::packet::handler::mob_respawn::error::MobRespawnError;
use crate::packet::handler::mob_respawn::store::MobRespawnStore;
use crate::packet::handler::result::HandlerResult;
use packet::model::Packet;
use crate::action::scope::BroadcastScope;
use state::model::SharedState;
use tick::manager::{MS_PER_TICK, TickManager};
use core::time::Duration;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::time::Instant;

pub struct MobRespawnHandler;

impl MobRespawnHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        tick_tx: broadcast::Sender<HandlerResult>,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
    ) -> Result<(), MobRespawnError> {
        let tick_tx_clone = tick_tx.clone();
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
                let store: MobRespawnStore = match MobRespawnStore::store_mob_respawn(
                    &state_clone,
                    now,
                    world_id,
                    channel_id,
                    map_wz,
                )
                .await
                {
                    Ok(respawn_store) => respawn_store,
                    Err(_) => continue,
                };
                let respawn_result: HandlerResult =
                    match MobRespawnHandler::build_respawn_result(&store).await {
                        Ok(respawn_result) => respawn_result,
                        Err(_) => continue,
                    };
                let _ = tick_tx_clone.send(respawn_result);
            }
        });
        Ok(())
    }

    pub async fn build_respawn_result(
        store: &MobRespawnStore,
    ) -> Result<HandlerResult, MobRespawnError> {
        let mut result: HandlerResult = HandlerResult::new();
        for (mob_id, mob_life) in store.to_respawn.clone() {
            let packet = Packet::new_empty()
                .build_spawn_mob_packet(mob_id, &mob_life)?
                .finish();
            result.add_action(Action::Broadcast(BroadcastAction::Send {
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
            result.add_action(Action::Broadcast(BroadcastAction::Send {
                packet: packet.clone(),
                scope: BroadcastScope::MapChar {
                    world_id: store.world_id,
                    channel_id: store.channel_id,
                    map_wz: store.map_wz,
                },
            }));
        }
        Ok(result)
    }
}
