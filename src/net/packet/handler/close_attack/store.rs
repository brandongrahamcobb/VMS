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

use crate::models::character::wrapper::Character;
use crate::models::skill;
use crate::models::skill::wrapper::Skill;
use crate::net::error::NetworkError;
use crate::net::packet::handler::close_attack::reader::CloseAttackReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CloseAttackStore {
    pub char: Character,
    pub skill: Skill,
    pub count: i16,
    pub display: i16,
    pub toleft: i16,
    pub stance: i16,
    pub speed: i16,
    pub mob_damages: HashMap<i32, Vec<i32>>,
}

impl CloseAttackStore {
    pub async fn store_close_attack(
        state: &SharedState,
        session: Session,
        reader: CloseAttackReader,
    ) -> Result<Self, NetworkError> {
        let char = session.get_active_char(state).await?;
        let skill_model = skill::query::getters::get_skill_model_by_character_id_and_skill_id(
            state,
            char.model.get_id()?,
            reader.skill_id,
        )
        .await?;
        let skill = skill_model.load()?;
        return Ok(Self {
            char: char.clone(),
            skill: skill.clone(),
            count: reader.count,
            display: reader.display,
            toleft: reader.toleft,
            stance: reader.stance,
            speed: reader.speed,
            mob_damages: reader.mob_damages.clone(),
        });
    }
}
