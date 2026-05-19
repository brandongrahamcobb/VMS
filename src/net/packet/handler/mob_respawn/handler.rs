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

use crate::net::action::{Action, BroadcastAction};
use crate::net::packet::handler::mob_respawn::error::MobRespawnError;
use crate::net::packet::handler::mob_respawn::store::MobRespawnStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::execute;
use crate::runtime::relay::scope::BroadcastScope;
use crate::runtime::state::SharedState;
use crate::runtime::tick::{MS_PER_TICK, TickManager};
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
                let result: HandlerResult =
                    match MobRespawnHandler::build_respawn_result(&store).await {
                        Ok(respawn_result) => respawn_result,
                        Err(_) => continue,
                    };
                let _ = tick_tx_clone.send(result);
            }
        });
        Ok(())
    }

    pub async fn build_respawn_result(
        store: &MobRespawnStore,
    ) -> Result<HandlerResult, MobRespawnError> {
        let mut result: HandlerResult = HandlerResult::new();
        for (mob_id, mob) in store.to_respawn.iter() {
            let packet = Packet::new_empty().build_spawn_mob_packet(&mob)?.finish();
            result.add_action(Action::Broadcast(BroadcastAction::Send {
                packet: packet.clone(),
                scope: BroadcastScope::Map {
                    world_id: store.world_id,
                    channel_id: store.channel_id,
                    map_wz: store.map_wz,
                },
            }));
            let packet = Packet::new_empty()
                .build_mob_move_packet(
                    *mob_id as u32,
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
        Ok(result)
    }
}
