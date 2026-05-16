/* create_char/store.rs
 * The purpose of this module is to resolve relevant variables for character creation.
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

use crate::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::db::error::DatabaseError;
use crate::models::character;
use crate::models::character::model::CharacterModel;
use crate::models::character::wrapper::Character;
use crate::models::item;
use crate::models::item::error::ItemError;
use crate::models::item::wrapper::Inventory;
use crate::models::job::model::JobModel;
use crate::models::job::wrapper::Job;
use crate::models::keybinding;
use crate::models::keybinding::model::KeybindingModel;
use crate::models::keybinding::wrapper::Keybinding;
use crate::models::map;
use crate::models::skill;
use crate::models::skill::model::SkillModel;
use crate::models::skill::wrapper::Skill;
use crate::net::packet::handler::create_char::error::CreateCharError;
use crate::net::packet::handler::create_char::reader::CreateCharReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use itertools::izip;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct CreateCharStore {
    pub char: Character,
}

impl CreateCharStore {
    async fn init_char_model(
        state: &SharedState,
        reader: &CreateCharReader,
        acc_id: i32,
        map_wz: i32,
        world_id: i16,
    ) -> Result<CharacterModel, CreateCharError> {
        let char_models: Vec<CharacterModel> = Vec::from([CharacterModel {
            id: None,
            acc_id,
            ign: reader.ign.clone(),
            job_wz: reader.job_wz,
            face_wz: reader.face_wz,
            hair_wz: reader.hair_wz,
            hair_color_wz: reader.hair_color_wz,
            skin_wz: reader.skin_wz,
            gender_wz: reader.gender_wz,
            map_wz: map_wz,
            world_id: world_id,
            level: 1,
            exp: 0,
            strength: 4,
            dexterity: 4,
            luck: 4,
            intelligence: 4,
            hp: 50,
            mp: 5,
            max_hp: 50,
            max_mp: 5,
            ap: 0,
            fame: 0,
            meso: 0,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        }]);
        let char_models = character::query::setters::update_characters(state, char_models)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        Ok(char_models[0].clone())
    }

    pub async fn init_keybindings(
        state: &SharedState,
        char_id: i32,
    ) -> Result<HashMap<i32, Keybinding>, CreateCharError> {
        let bind_models: Vec<KeybindingModel> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
            .map(|(key, bind_type, action)| KeybindingModel {
                action,
                bind_type,
                char_id,
                created_at: Some(SystemTime::now()),
                key,
                updated_at: SystemTime::now(),
            })
            .collect();
        let bind_models = keybinding::query::setters::update_keybindings(state, bind_models)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        Ok(bind_models
            .into_iter()
            .map(|b| -> Result<(i32, Keybinding), CreateCharError> { Ok((b.key, b.load()?)) })
            .collect::<Result<HashMap<i32, Keybinding>, CreateCharError>>()?)
    }

    async fn init_equips(
        state: &SharedState,
        reader: &CreateCharReader,
        char_id: i32,
    ) -> Result<Inventory, CreateCharError> {
        let mut inventory: Inventory = item::service::load_inventory(state, char_id).await?;
        let top = item::service::create_item(state, reader.top_wz).await?;
        let top = {
            let pos = inventory.pick_up(state, char_id, top).await?;
            inventory
                .equip_tab
                .remove(&pos)
                .ok_or(ItemError::InvalidISlot)?
        };
        inventory.equip(state, top).await?;
        let bottom = item::service::create_item(state, reader.bottom_wz).await?;
        let bottom = {
            let pos = inventory.pick_up(state, char_id, bottom).await?;
            inventory
                .equip_tab
                .remove(&pos)
                .ok_or(ItemError::InvalidISlot)?
        };
        inventory.equip(state, bottom).await?;
        let shoes = item::service::create_item(state, reader.shoes_wz).await?;
        let shoes = {
            let pos = inventory.pick_up(state, char_id, shoes).await?;
            inventory
                .equip_tab
                .remove(&pos)
                .ok_or(ItemError::InvalidISlot)?
        };
        inventory.equip(state, shoes).await?;
        let weapon = item::service::create_item(state, reader.weapon_wz).await?;
        let weapon = {
            let pos = inventory.pick_up(state, char_id, weapon).await?;
            inventory
                .equip_tab
                .remove(&pos)
                .ok_or(ItemError::InvalidISlot)?
        };
        inventory.equip(state, weapon).await?;
        Ok(inventory)
    }

    pub async fn init_skills(
        state: &SharedState,
        reader: &CreateCharReader,
        char_id: i32,
    ) -> Result<HashMap<i32, Skill>, CreateCharError> {
        let skill_models: Vec<SkillModel> =
            skill::service::generate_skill_wzs_by_job_wz(reader.job_wz as i32)?
                .into_iter()
                .map(|wz| SkillModel {
                    char_id,
                    created_at: Some(SystemTime::now()),
                    level: 0,
                    updated_at: SystemTime::now(),
                    wz,
                })
                .collect();
        skill::query::setters::update_skills(state, skill_models.clone())
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        Ok(skill_models
            .into_iter()
            .map(|s| -> Result<(i32, Skill), CreateCharError> { Ok((s.wz, s.load()?)) })
            .collect::<Result<HashMap<i32, Skill>, CreateCharError>>()?)
    }

    pub async fn store_create_char(
        state: &SharedState,
        session: &Session,
        reader: &CreateCharReader,
    ) -> Result<Self, CreateCharError> {
        let acc_id: i32 = session.get_acc_id()?;
        let world_id: i16 = session.get_world_id()?;
        let map_wz: i32 = map::service::get_map_wz_by_job_id(reader.job_wz)?;
        let job = Job {
            model: JobModel { wz: reader.job_wz },
        };
        let char_model = Self::init_char_model(state, reader, acc_id, map_wz, world_id).await?;
        let char_id = char_model.get_id()?;
        let binds: HashMap<i32, Keybinding> = Self::init_keybindings(state, char_id).await?;
        let inventory = Self::init_equips(state, reader, char_id).await?;
        let skills: HashMap<i32, Skill> = Self::init_skills(state, reader, char_id).await?;
        let char = Character {
            model: char_model,
            binds,
            job,
            inventory,
            skills,
        };
        Ok(Self { char })
    }
}
