/* db/src/character/model.rs
 * The purpose of this module is to provide a character db model and associated methods.
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
use crate::character::error::CharacterModelError;
use crate::schema::characters;
use diesel::prelude::*;
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
    pub hp: i32,
    pub mp: i16,
    pub max_hp: i32,
    pub max_mp: i16,
    pub ap: i16,
    pub sp: i16,
    pub fame: i16,
    pub meso: i32,
    pub job_wz: i16,
    pub face_wz: i32,
    pub hair_wz: i32,
    pub hair_color_wz: i32,
    pub skin_wz: i32,
    pub gender_wz: i16,
    pub last_portal: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl CharacterModel {
    pub fn default(
        acc_id: i32,
        world_id: i16,
        map_wz: i32,
        ign: String,
        job_wz: i16,
        face_wz: i32,
        hair_wz: i32,
        hair_color_wz: i32,
        skin_wz: i32,
        gender_wz: i16,
    ) -> CharacterModel {
        CharacterModel {
            id: None,
            acc_id,
            world_id,
            map_wz,
            ign,
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
            job_wz,
            face_wz,
            hair_wz,
            hair_color_wz,
            skin_wz,
            gender_wz,
            last_portal: 0,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        }
    }

    pub fn get_id(&self) -> Result<i32, CharacterModelError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(CharacterModelError::NoId)
        }
    }

    pub fn get_created_at(&self) -> Result<SystemTime, CharacterModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(CharacterModelError::NoCreatedAt(self.get_id()?))
        }
    }
}
