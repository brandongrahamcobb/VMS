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

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct MapleCharacter {
    pub id: i32,
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
}

#[derive(Component)]
pub struct InChar(pub Entity);

impl From<Character> for MapleCharacter {
    fn from(c: Character) -> Self {
        Self {
            id: c.id,
            ign: c.ign,
            level: c.level,
            exp: c.exp,
            strength: c.stregth,
            dexterity: c.dexterity,
            luck: c.luck,
            intelligence: c.intelligence,
            hp: c.hp,
            mp: c.mp,
            max_hp: c.max_hp,
            max_mp: c.max_mp,
            ap: c.ap,
            sp: c.sp,
            fame: c.fame,
            meso: c.meso,
            job_wz: c.job_wz,
            face_wz: c.face_wz,
            hair_wz: c.hair_wz,
            hair_color_wz: c.hair_color_wz,
            skin_wz: c.skin_wz,
            gender_wz: c.gender_wz,
            last_portal: c.last_portal,
        }
    }
}
