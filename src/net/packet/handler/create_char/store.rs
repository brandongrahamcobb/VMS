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
use crate::models::account::wrapper::Account;
use crate::models::channel::wrapper::Channel;
use crate::models::character;
use crate::models::character::model::CharacterModel;
use crate::models::character::wrapper::Character;
use crate::models::item;
use crate::models::item::wrapper::Item;
use crate::models::keybinding;
use crate::models::keybinding::model::KeybindingModel;
use crate::models::keybinding::wrapper::Keybinding;
use crate::models::map;
use crate::models::map::wrapper::Map;
use crate::models::skill;
use crate::models::skill::model::SkillModel;
use crate::models::skill::wrapper::Skill;
use crate::models::world::wrapper::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::create_char::reader::CreateCharReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use itertools::izip;
use std::time::SystemTime;

#[derive(Clone)]
pub struct CreateCharStore {
    pub char: Character,
}

impl CreateCharStore {
    async fn init_char_model(
        state: &SharedState,
        reader: CreateCharReader,
        acc: Account,
        map: Map,
        world: World,
    ) -> Result<CharacterModel, NetworkError> {
        let char_models: Vec<CharacterModel> = Vec::from([CharacterModel {
            id: None,
            acc_id: acc.model.get_id()?,
            ign: reader.ign.clone(),
            job_wz: reader.job_wz,
            face_wz: reader.face_wz,
            hair_wz: reader.hair_wz,
            hair_color_wz: reader.hair_color_wz,
            skin_wz: reader.skin_wz,
            gender_wz: reader.gender_wz,
            map_wz: map.model.wz,
            world_id: world.model.id,
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
        let char_models = character::query::setters::update_characters(state, char_models).await?;
        Ok(char_models[0].clone())
    }

    pub async fn init_keybindings(
        state: &SharedState,
        char_id: i32,
    ) -> Result<Vec<Keybinding>, NetworkError> {
        let bind_models: Vec<KeybindingModel> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
            .map(
                |(key, bind_type, action): (i32, i16, i32)| KeybindingModel {
                    char_id,
                    key,
                    bind_type,
                    action,
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                },
            )
            .collect();
        let bind_models =
            keybinding::query::setters::update_keybindings(state, bind_models).await?;
        let mut binds: Vec<Keybinding> = Vec::<Keybinding>::new();
        for bind_model in &bind_models {
            binds.push(bind_model.load()?);
        }
        Ok(binds.clone())
    }

    async fn init_equips(
        state: &SharedState,
        reader: CreateCharReader,
        char_id: i32,
    ) -> Result<Vec<Item>, NetworkError> {
        let mut equips: Vec<Item> = Vec::<Item>::new();
        let equipped: bool = true;
        let top = item::service::create_item(state, Some(char_id), equipped, reader.top_wz).await?;
        equips.push(top);
        let bottom =
            item::service::create_item(state, Some(char_id), equipped, reader.bottom_wz).await?;
        equips.push(bottom);
        let shoes =
            item::service::create_item(state, Some(char_id), equipped, reader.shoes_wz).await?;
        equips.push(shoes);
        let weapon =
            item::service::create_item(state, Some(char_id), equipped, reader.weapon_wz).await?;
        equips.push(weapon);
        Ok(equips)
    }

    pub async fn init_skills(
        state: &SharedState,
        reader: CreateCharReader,
        char_id: i32,
    ) -> Result<Vec<Skill>, NetworkError> {
        let mut skill_models: Vec<SkillModel> = Vec::<SkillModel>::new();
        let skill_wzs: Vec<i32> =
            skill::service::generate_skill_wzs_by_job_wz(reader.job_wz as i32)?;
        for skill_wz in skill_wzs {
            skill_models.push(SkillModel {
                char_id,
                wz: skill_wz,
                level: 0,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            });
        }
        skill::query::setters::update_skills(state, skill_models.clone()).await?;
        let mut skills: Vec<Skill> = Vec::<Skill>::new();
        for skill_model in skill_models {
            skills.push(skill_model.load()?);
        }
        Ok(skills)
    }

    pub async fn store_create_char(
        state: &SharedState,
        session: Session,
        reader: CreateCharReader,
    ) -> Result<Self, NetworkError> {
        let acc: Account = session.get_acc()?;
        let channel: Channel = session.get_active_channel(state).await?;
        let world: World = session.get_active_world(state).await?;
        let map_wz = map::service::get_map_by_job_wz(reader.job_wz)?;
        let map = map::service::get_map_by_world_channel_map_wzs(
            state,
            world.model.id,
            channel.model.id,
            map_wz,
        )
        .await?;
        let char_model = Self::init_char_model(
            state,
            reader.clone(),
            acc.clone(),
            map.clone(),
            world.clone(),
        )
        .await?;
        let char_id = char_model.get_id()?;
        let binds: Vec<Keybinding> = Self::init_keybindings(state, char_id).await?;
        let items = Self::init_equips(state, reader.clone(), char_id).await?;
        let skills = Self::init_skills(state, reader.clone(), char_id).await?;
        let char = Character {
            model: char_model,
            binds,
            items,
            skills,
        };
        Ok(Self { char })
    }
}
