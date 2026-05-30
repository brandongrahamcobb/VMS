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

use crate::create_char::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::create_char::error::CreateCharError;
use crate::create_char::reader::CreateCharReader;
use db;
use db::pool::DbPool;
use entity::character::model::CharacterModel;
use entity::character::wrapper::Character;
use entity::item::model::ItemModel;
use entity::item::wrapper::{Inventory, Item};
use entity::job::model::JobWzSkill;
use entity::job::wrapper::Job;
use entity::keybinding::model::{KeybindType, KeybindingModel};
use entity::keybinding::wrapper::Keybinding;
use entity::map::model::Point;
use entity::skill::model::SkillModel;
use entity::skill::wrapper::Skill;
use itertools::izip;
use session::model::Session;
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

pub struct CreateCharEvent {
    pub char: Character,
}

pub struct CreateCharCommand {
    pub client_id: i32,
    pub acc_id: i32,
    pub world_id: i16,
    pub ign: String,
    pub job_wz: i16,
    pub face_wz: i32,
    pub hair_wz: i32,
    pub hair_color_wz: i32,
    pub skin_wz: i32,
    pub top_wz: i32,
    pub bottom_wz: i32,
    pub shoes_wz: i32,
    pub weapon_wz: i32,
    pub gender_wz: i16,
}

impl CreateCharEvent {
    async fn init_char_model(
        pool: &DbPool,
        reader: &CreateCharReader,
        acc_id: i32,
        world_id: i16,
        map_wz: i32,
    ) -> Result<CharacterModel, CreateCharError> {
        let char_model: CharacterModel = CharacterModel {
            id: None,
            acc_id,
            ign: reader.ign.clone(),
            job_wz: reader.job_wz,
            face_wz: reader.face_wz,
            hair_wz: reader.hair_wz,
            hair_color_wz: reader.hair_color_wz,
            skin_wz: reader.skin_wz,
            gender_wz: reader.gender_wz,
            map_wz,
            world_id,
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
            sp: 0,
            fame: 0,
            meso: 0,
            last_portal: 0,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        };
        let char_models = db::character::setters::update_characters(pool, vec![char_model]).await?;
        Ok(char_models[0].clone())
    }

    pub async fn init_keybindings(
        pool: &DbPool,
        char_id: i32,
    ) -> Result<HashMap<i32, Keybinding>, CreateCharError> {
        let mut bind_models: Vec<KeybindingModel> =
            izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
                .map(|(key, bind_type, action)| KeybindingModel {
                    id: None,
                    action,
                    bind_type,
                    char_id,
                    created_at: Some(SystemTime::now()),
                    key,
                    updated_at: SystemTime::now(),
                })
                .collect();
        let used_keys: HashSet<i32> = bind_models.iter().map(|b| b.key).collect();
        for key in 0i32..90 {
            if !used_keys.contains(&key) {
                bind_models.push(KeybindingModel {
                    id: None,
                    action: 0,
                    bind_type: KeybindType::Nil as i16,
                    char_id,
                    created_at: Some(SystemTime::now()),
                    key,
                    updated_at: SystemTime::now(),
                });
            }
        }
        let bind_models: Vec<KeybindingModel> =
            db::keybinding::setters::update_keybindings(pool, bind_models).await?;
        let mut binds: HashMap<i32, Keybinding> = HashMap::new();
        for bind_model in bind_models {
            binds.insert(
                bind_model.get_id()?,
                assembly::keybinding::assemble::assemble_keybinding_by_id(
                    pool,
                    bind_model.get_id()?,
                )
                .await?, // expensive
                         // and
                         // not
                         // neccesary
            );
        }
        Ok(binds)
    }

    async fn init_equips(
        pool: &DbPool,
        reader: &CreateCharReader,
        ign: String,
        char_id: i32,
    ) -> Result<Inventory, CreateCharError> {
        let mut inv: Inventory =
            assembly::item::assemble::assemble_inventory_by_char_id(pool, char_id).await?;
        let top_model: ItemModel =
            metadata::item::inventory::create_item_model_by_wz(reader.top_wz)?;
        let top_model: ItemModel = db::item::setters::update_item(pool, &top_model).await?;
        let top_id: i32 = top_model.get_id()?;
        let top: Item = assembly::item::assemble::assemble_item_by_id(pool, top_id).await?;
        domain::item::pick_up(pool, &mut inv, ign.clone(), char_id, top).await?;
        domain::item::equip(pool, &mut inv, top_id).await?;
        let bottom_model: ItemModel =
            metadata::item::inventory::create_item_model_by_wz(reader.bottom_wz)?;
        let bottom_model: ItemModel = db::item::setters::update_item(pool, &bottom_model).await?;
        let bottom_id: i32 = bottom_model.get_id()?;
        let bottom = assembly::item::assemble::assemble_item_by_id(pool, bottom_id).await?;
        domain::item::pick_up(pool, &mut inv, ign.clone(), char_id, bottom).await?;
        domain::item::equip(pool, &mut inv, bottom_id).await?;
        let shoes_model: ItemModel =
            metadata::item::inventory::create_item_model_by_wz(reader.shoes_wz)?;
        let shoes_model: ItemModel = db::item::setters::update_item(pool, &shoes_model).await?;
        let shoes_id: i32 = shoes_model.get_id()?;
        let shoes: Item = assembly::item::assemble::assemble_item_by_id(pool, shoes_id).await?;
        domain::item::pick_up(pool, &mut inv, ign.clone(), char_id, shoes).await?;
        domain::item::equip(pool, &mut inv, shoes_id).await?;
        let weapon_model: ItemModel =
            metadata::item::inventory::create_item_model_by_wz(reader.weapon_wz)?;
        let weapon_model: ItemModel = db::item::setters::update_item(pool, &weapon_model).await?;
        let weapon_id: i32 = weapon_model.get_id()?;
        let weapon: Item = assembly::item::assemble::assemble_item_by_id(pool, weapon_id).await?;
        domain::item::pick_up(pool, &mut inv, ign.clone(), char_id, weapon).await?;
        domain::item::equip(pool, &mut inv, weapon_id).await?;
        Ok(inv)
    }

    pub async fn init_skills(
        pool: &DbPool,
        char_id: i32,
        job_wz_skills: Vec<JobWzSkill>,
    ) -> Result<HashMap<i32, Skill>, CreateCharError> {
        let mut skill_models_insert: Vec<SkillModel> = Vec::new();
        let close_attack_wz: i32 = 0;
        skill_models_insert.push(SkillModel {
            id: None,
            char_id,
            level: 0,
            wz: close_attack_wz,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        });
        for job_wz_skill in job_wz_skills {
            skill_models_insert.push(SkillModel {
                id: None,
                char_id,
                level: 0,
                wz: job_wz_skill.wz,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            });
        }
        let skill_models: Vec<SkillModel> =
            db::skill::setters::update_skills(pool, skill_models_insert.clone()).await?;
        let mut skills: HashMap<i32, Skill> = HashMap::new();
        for skill_model in skill_models {
            skills.insert(
                skill_model.wz,
                assembly::skill::assemble::assemble_skill_by_id(pool, skill_model.get_id()?)
                    .await?,
            );
        }
        Ok(skills)
    }

    pub async fn store_create_char(
        pool: &DbPool,
        session: &Session,
        reader: &CreateCharReader,
    ) -> Result<Self, CreateCharError> {
        let acc_id: i32 = session.get_acc_id()?;
        let world_id: i16 = session.get_world_id()?;
        let map_wz: i32 = entity::map::service::get_map_wz_by_job_id(reader.job_wz);
        let char_model = Self::init_char_model(pool, reader, acc_id, world_id, map_wz).await?;
        let char_id = char_model.get_id()?;
        let binds: HashMap<i32, Keybinding> = Self::init_keybindings(pool, char_id).await?;
        let inventory = Self::init_equips(pool, reader, char_model.ign.clone(), char_id).await?;
        let job: Job = assembly::job::assemble::assemble_job_by_wz(reader.job_wz)?;
        let skills: HashMap<i32, Skill> =
            Self::init_skills(pool, char_id, job.info.skills.clone()).await?;
        let pos: Point = metadata::map::portal::get_zeroeth_portal_spawnpoint(map_wz)?;
        let char = Character {
            model: char_model,
            binds,
            job,
            inventory,
            skills,
            pos,
        };
        Ok(Self { char })
    }
}
