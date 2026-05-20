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

use tokio::time::Instant;

use crate::db::error::DatabaseError;
use crate::models::item::wrapper::Item;
use crate::models::mob::wrapper::{DeathState, LifeState, Mob};
use crate::models::{item, skill};
use crate::net::packet::handler::close_attack::error::CloseAttackError;
use crate::net::packet::handler::close_attack::reader::CloseAttackReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

pub struct CloseAttackStore {
    pub char_id: i32,
    pub dead_mobs: HashMap<u32, Mob>,
    pub dead_mobs_mesos: HashMap<u32, i32>,
    pub dead_mobs_drops: HashMap<u32, HashMap<i32, Item>>,
    pub skill_level: i16,
    pub skill_wz: i32,
    pub count: i16,
    pub display: i16,
    pub toleft: i16,
    pub stance: i16,
    pub speed: i16,
    pub hp_updates: HashMap<u32, i16>,
    pub mob_damages: HashMap<u32, Vec<i32>>,
    pub world_id: i16,
    pub channel_id: u8,
    pub map_wz: i32,
    pub mode: u8,
    pub owner: i32,
    pub can_pickup: u8,
    pub player_drop: bool,
}

impl CloseAttackStore {
    pub async fn store_close_attack(
        state: &SharedState,
        session: &Session,
        reader: &CloseAttackReader,
    ) -> Result<Self, CloseAttackError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        let char_id: i32 = session.get_char_id()?;
        let skill_model = skill::query::getters::get_skill_model_by_character_id_and_skill_id(
            state,
            char_id,
            reader.skill_id,
        )
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
        let hp_updates: HashMap<u32, i16> = {
            let state = state.lock().await;
            state
                .with_mut_map(
                    world_id,
                    channel_id,
                    map_wz,
                    |map| -> Result<HashMap<u32, i16>, CloseAttackError> {
                        let mut hp_updates: HashMap<u32, i16> = HashMap::new();
                        for (mob_id, damage) in reader.mob_damages.iter() {
                            if let Some(mob) = map.mobs.get_mut(mob_id) {
                                let new_hp = {
                                    let total_damage: i32 = damage.iter().sum();
                                    mob.model.hp -= total_damage;
                                    mob.model.hp
                                };
                                let hp_percent = (new_hp * 100 / mob.info.max_hp) as i16;
                                hp_updates.insert(*mob_id, hp_percent);
                            }
                        }
                        Ok(hp_updates)
                    },
                )
                .await??
        };
        let dead_mobs: HashMap<u32, Mob> = {
            let state = state.lock().await;
            state
                .with_mut_map(
                    world_id,
                    channel_id,
                    map_wz,
                    |map| -> Result<HashMap<u32, Mob>, CloseAttackError> {
                        let mut dead_mobs: HashMap<u32, Mob> = HashMap::new();
                        for (mob_id, hp_percent) in hp_updates.iter() {
                            if let Some(mob) = map.mobs.get_mut(mob_id) {
                                if *hp_percent == 0 {
                                    let life_state: LifeState = LifeState::Dead(DeathState {
                                        died_at: Instant::now(),
                                    });
                                    mob.life_state = life_state.clone();
                                    dead_mobs.insert(
                                        *mob_id,
                                        Mob {
                                            model: mob.model.clone(),
                                            info: mob.info.clone(),
                                            life: mob.life.clone(),
                                            life_state,
                                        },
                                    );
                                }
                            }
                        }
                        Ok(dead_mobs)
                    },
                )
                .await??
        };
        let mut dead_mobs_drops: HashMap<u32, HashMap<i32, Item>> = HashMap::new();
        let mut dead_mobs_mesos: HashMap<u32, i32> = HashMap::new();
        for (mob_id, mob) in dead_mobs.iter() {
            let drops: HashMap<i32, Item> =
                item::service::get_random_drops(state, mob.life.clone()).await?;
            let mesos: i32 = item::service::get_random_meso_drop(mob.info.clone()).await?;
            dead_mobs_drops.insert(*mob_id, drops);
            dead_mobs_mesos.insert(*mob_id, mesos);
        }
        let mode: u8 = 1; // animation 0 fade, 1 drop mob, 2 spawn in
        let owner: i32 = 0; // char id or 0
        let can_pickup: u8 = 0; // 0 everyone 1 owner, 2 party
        let player_drop: bool = false;
        return Ok(Self {
            char_id,
            dead_mobs,
            dead_mobs_drops,
            dead_mobs_mesos,
            skill_level: skill_model.level,
            skill_wz: skill_model.wz,
            count: reader.count,
            display: reader.display,
            toleft: reader.toleft,
            stance: reader.stance,
            speed: reader.speed,
            hp_updates,
            mob_damages: reader.mob_damages.clone(),
            world_id,
            channel_id,
            map_wz,
            mode,
            owner,
            can_pickup,
            player_drop,
        });
    }
}
