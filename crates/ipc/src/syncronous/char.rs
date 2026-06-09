/* ipc/src/sync/char.rs
 * The purpose of this module is to provide constants for syncronous logic for characters.
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

use std::collections::HashSet;
use std::time::SystemTime;

use crate::syncronous::error::SyncDomainError;
use base::inventory::REGULAR_EQUIP_SLOTS;
use base::keybinding::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE, KeybindType};
use db::character::model::CharacterModel;
use db::item::model::ItemModel;
use db::keybinding::model::KeybindingModel;
use db::skill::model::SkillModel;
use itertools::izip;

pub fn create_new_char_model(
    acc_id: i32,
    world_id: i16,
    ign: String,
    job_wz: i16,
    face_wz: i32,
    hair_wz: i32,
    hair_color_wz: i32,
    skin_wz: i32,
    gender_wz: i16,
) -> CharacterModel {
    let map_wz: i32 = crate::syncronous::map::get_map_wz_by_job_id(job_wz);
    let char_model: CharacterModel = CharacterModel::default(
        acc_id,
        world_id,
        map_wz,
        ign,
        job_wz,
        face_wz,
        hair_wz,
        hair_color_wz,
        skin_wz,
        gender_wz,
    );
    char_model
}

pub fn create_new_char_keybinding_models(char_id: i32) -> Vec<KeybindingModel> {
    let mut bind_models: Vec<KeybindingModel> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
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
    bind_models
}

pub fn create_new_char_equip_models(
    char_id: i32,
    top_wz: i32,
    bottom_wz: i32,
    shoes_wz: i32,
    weapon_wz: i32,
) -> Result<Vec<ItemModel>, SyncDomainError> {
    let mut equips: Vec<ItemModel> = Vec::new();
    let mut top_model: ItemModel = ItemModel::default(char_id, top_wz)?;
    top_model.equipped = true;
    top_model.ipos = REGULAR_EQUIP_SLOTS
        .iter()
        .find(|islot| islot.name == "Top")
        .map(|islot| islot.key);
    equips.push(top_model);
    let mut bottom_model: ItemModel = ItemModel::default(char_id, bottom_wz)?;
    bottom_model.equipped = true;
    bottom_model.ipos = REGULAR_EQUIP_SLOTS
        .iter()
        .find(|islot| islot.name == "Bottom")
        .map(|islot| islot.key);
    equips.push(bottom_model);
    let mut shoes_model: ItemModel = ItemModel::default(char_id, shoes_wz)?;
    shoes_model.equipped = true;
    shoes_model.ipos = REGULAR_EQUIP_SLOTS
        .iter()
        .find(|islot| islot.name == "Shoes")
        .map(|islot| islot.key);
    equips.push(shoes_model);
    let mut weapon_model: ItemModel = ItemModel::default(char_id, weapon_wz)?;
    weapon_model.equipped = true;
    weapon_model.ipos = REGULAR_EQUIP_SLOTS
        .iter()
        .find(|islot| islot.name == "Weapon")
        .map(|islot| islot.key);
    equips.push(weapon_model);
    Ok(equips)
}

pub fn create_new_char_skill_models(
    char_id: i32,
    job_wz: i16,
) -> Result<Vec<SkillModel>, SyncDomainError> {
    let mut skill_models: Vec<SkillModel> = Vec::new();
    let close_attack_wz: i32 = 0;
    skill_models.push(SkillModel {
        id: None,
        char_id,
        level: 0,
        wz: close_attack_wz,
        created_at: Some(SystemTime::now()),
        updated_at: SystemTime::now(),
    });
    let skill_wzs: Vec<i32> = metadata::job::skill::get_job_skill_wzs_by_job_wz(job_wz)?;
    for skill_wz in skill_wzs {
        skill_models.push(SkillModel {
            id: None,
            char_id,
            level: 0,
            wz: skill_wz,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        });
    }
    Ok(skill_models)
}
