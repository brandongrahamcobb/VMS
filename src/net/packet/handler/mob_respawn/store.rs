/* mob_respawn/store.rs
 * The purpose of this module is to store relevant values for mob respawning.
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

use crate::models::mob::wrapper::Mob;
use crate::net::packet::handler::mob_respawn::error::MobRespawnError;
use crate::runtime::state::SharedState;
use std::collections::HashMap;
use tokio::time::Instant;

pub struct MobRespawnStore {
    pub to_respawn: HashMap<u32, Mob>,
    pub world_id: i16,
    pub channel_id: u8,
    pub map_wz: i32,
}

impl MobRespawnStore {
    pub async fn store_mob_respawn(
        state: &SharedState,
        now: Instant,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
    ) -> Result<Self, MobRespawnError> {
        let to_respawn: HashMap<u32, Mob> = {
            let state = state.lock().await;
            state
                .with_mut_map(
                    world_id,
                    channel_id,
                    map_wz,
                    |map| -> Result<HashMap<u32, Mob>, MobRespawnError> {
                        let due: Vec<u32> = map
                            .dead_mobs
                            .iter()
                            .filter(|(_, dead)| {
                                now.duration_since(dead.died_at) >= dead.respawn_time
                            })
                            .map(|(mob_id, _)| *mob_id)
                            .collect();
                        let mut to_respawn = HashMap::new();
                        for mob_id in due {
                            if let Some(mut dead) = map.dead_mobs.remove(&mob_id) {
                                dead.model.hp = dead.model.max_hp;
                                let map_mob = Mob {
                                    model: dead.model.clone(),
                                };
                                map.mobs.insert(mob_id, map_mob);
                                let respawned_mob = Mob {
                                    model: dead.model.clone(),
                                };
                                to_respawn.insert(mob_id, respawned_mob);
                            }
                        }
                        Ok(to_respawn)
                    },
                )
                .await??
        };
        Ok(Self {
            to_respawn,
            world_id,
            channel_id,
            map_wz,
        })
    }
}
