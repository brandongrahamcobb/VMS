/* character/model.rs
 * The purpose of this module is to provide a character model and its wrapper.
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
use crate::db::schema::character_limits;
use crate::db::schema::characters;
use crate::models::character::error::CharacterError;
use crate::models::character::wrapper::Character;
use crate::models::error::ModelError;
use crate::models::item;
use crate::models::item::model::Inventory;
use crate::models::job;
use crate::models::job::wrapper::Job;
use crate::models::keybinding;
use crate::models::keybinding::wrapper::Keybinding;
use crate::models::skill;
use crate::models::skill::wrapper::Skill;
use crate::runtime::state::SharedState;
use diesel::prelude::*;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Clone, Identifiable, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = characters)]
pub struct CharacterModel {
    pub id: Option<i32>,
    pub acc_id: i32,
    pub world_id: i16,
    pub map_wz: i32,
    pub ign: String,
    pub level: i16,
    pub exp: i32,
    pub strength: i16,
    pub dexterity: i16,
    pub luck: i16,
    pub intelligence: i16,
    pub hp: i16,
    pub mp: i16,
    pub max_hp: i16,
    pub max_mp: i16,
    pub ap: i16,
    pub fame: i16,
    pub meso: i32,
    pub job_wz: i16,
    pub face_wz: i32,
    pub hair_wz: i32,
    pub hair_color_wz: i32,
    pub skin_wz: i32,
    pub gender_wz: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

#[derive(Queryable, AsChangeset)]
#[diesel(table_name = character_limits)]
pub struct CharacterLimitModel {
    pub acc_id: i32,
    pub char_max: i16,
    pub world_id: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl CharacterModel {
    pub async fn load(&self, state: &SharedState) -> Result<Character, ModelError> {
        let char_id: i32 = self.get_id()?;
        let binds: HashMap<i32, Keybinding> =
            keybinding::service::load_keybindings(state, char_id).await?;
        let inventory: Inventory = item::service::load_inventory(state, char_id).await?;
        let job: Job = job::service::load_job(char_id).await?;
        let skills: HashMap<i32, Skill> = skill::service::load_skills(state, char_id).await?;
        Ok(Character {
            model: self.clone(),
            binds,
            inventory,
            job,
            skills,
        })
    }
    pub fn get_id(&self) -> Result<i32, ModelError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(ModelError::from(CharacterError::NoId))
        }
    }
}
