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

use crate::mob_respawn::error::MobRespawnError;
use core::time::Duration;
use entity::mob::model::{LifeState, MobWzLife};
use state::model::SharedState;
use std::collections::HashMap;
use tokio::time::Instant;

pub struct MobRespawnStore {
    pub to_respawn: HashMap<u32, MobWzLife>,
    pub world_id: i16,
    pub channel_id: u8,
    pub map_wz: i32,
    pub mode: i8,
    pub stance: i8,
    pub effect: i8,
    pub team: i8,
}

impl MobRespawnStore {
    pub async fn store_mob_respawn(
        state: &SharedState,
        now: Instant,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
    ) -> Result<Self, MobRespawnError> {
        let to_respawn: HashMap<u32, MobWzLife> = {
            let state = state.lock().await;
            state
                .with_mut_map(
                    world_id,
                    channel_id,
                    map_wz,
                    |map| -> Result<HashMap<u32, MobWzLife>, MobRespawnError> {
                        let due: Vec<u32> = map
                            .mobs
                            .iter()
                            .filter(|(_, dead)| match &dead.life_state {
                                LifeState::Dead(death_state) => {
                                    now.duration_since(death_state.died_at)
                                        >= Duration::from_secs(dead.life.mob_time)
                                }
                                LifeState::Alive => false,
                            })
                            .map(|(mob_id, _)| *mob_id)
                            .collect();
                        let mut to_respawn = HashMap::new();
                        for mob_id in due {
                            if let Some(dead) = map.mobs.get_mut(&mob_id) {
                                dead.model.hp = dead.info.max_hp;
                                dead.life_state = LifeState::Alive;
                                to_respawn.insert(dead.model.id, dead.life.clone());
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
            mode: 1,
            stance: 0,
            effect: 0,
            team: -1,
        })
    }
}
