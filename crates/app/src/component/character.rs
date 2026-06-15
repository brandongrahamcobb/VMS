/* app/src/component/character.rs
 * The purpose of this module is to provide a character component.
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

use core::convert::From;

use bevy::ecs::{component::Component, entity::Entity};
use db::character::model::CharacterModel;

#[derive(Clone, Component, Eq, Hash, PartialEq)]
pub struct MapleCharacter {
    pub id: i32,
    pub ign: String,
    pub level: i16,
    pub exp: i32,
    pub strength: i16,
    pub dexterity: i16,
    pub luck: i16,
    pub intelligence: i16,
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
    pub world_id: i16,
    pub spawn_map_wz: i32,
}

#[derive(Component)]
pub struct InChar(pub Entity);

impl From<CharacterModel> for MapleCharacter {
    fn from(model: CharacterModel) -> Self {
        let id = if let Some(id) = model.id { id } else { 0 };
        Self {
            id,
            ign: model.ign,
            level: model.level,
            exp: model.exp,
            strength: model.strength,
            dexterity: model.dexterity,
            luck: model.luck,
            intelligence: model.intelligence,
            ap: model.ap,
            sp: model.sp,
            fame: model.fame,
            meso: model.meso,
            job_wz: model.job_wz,
            face_wz: model.face_wz,
            hair_wz: model.hair_wz,
            hair_color_wz: model.hair_color_wz,
            skin_wz: model.skin_wz,
            gender_wz: model.gender_wz,
            last_portal: model.last_portal,
            world_id: model.world_id,
            spawn_map_wz: model.map_wz,
        }
    }
}
