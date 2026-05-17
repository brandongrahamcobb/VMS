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

use crate::db::error::DatabaseError;
use crate::models::mob::error::MobError;
use crate::models::{mob, skill};
use crate::net::packet::handler::close_attack::error::CloseAttackError;
use crate::net::packet::handler::close_attack::reader::CloseAttackReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

pub struct CloseAttackStore {
    pub char_id: i32,
    pub dead_mobs: Vec<u32>,
    pub skill_level: i16,
    pub skill_wz: i32,
    pub count: i16,
    pub display: i16,
    pub toleft: i16,
    pub stance: i16,
    pub speed: i16,
    pub hp_updates: HashMap<u32, i16>,
    pub mob_damages: HashMap<u32, Vec<i32>>,
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
        let (dead_mobs, hp_updates): (Vec<u32>, HashMap<u32, i16>) = {
            let state = state.lock().await;
            state
                .with_mut_map(
                    world_id,
                    channel_id,
                    map_wz,
                    |map| -> Result<_, CloseAttackError> {
                        let mut dead_mobs: Vec<u32> = Vec::new();
                        let mut hp_updates: HashMap<u32, i16> = HashMap::new();
                        for (mob_id, damage) in reader.mob_damages.iter() {
                            let mob = map
                                .mobs
                                .get_mut(mob_id)
                                .ok_or(MobError::NotFound(*mob_id))?;
                            let total_damage: i32 = damage.iter().sum();
                            mob.model.hp -= total_damage;
                            if mob.model.hp > 0 {
                                let hp_percent: i16 =
                                    (mob.model.hp * 100 / mob.model.max_hp) as i16;
                                hp_updates.insert(mob.model.id, hp_percent);
                            } else {
                                dead_mobs.push(mob.model.id);
                            }
                        }
                        for mob_id in dead_mobs.iter() {
                            map.mobs.remove(&mob_id);
                        }
                        Ok((dead_mobs, hp_updates))
                    },
                )
                .await??
        };
        return Ok(Self {
            char_id,
            dead_mobs,
            skill_level: skill_model.level,
            skill_wz: skill_model.wz,
            count: reader.count,
            display: reader.display,
            toleft: reader.toleft,
            stance: reader.stance,
            speed: reader.speed,
            hp_updates,
            mob_damages: reader.mob_damages.clone(),
        });
    }
}
