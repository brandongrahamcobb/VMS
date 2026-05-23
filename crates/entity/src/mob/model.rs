/* entity/src/mob/model.rs
 * The purpose of this module is to provide mob models.
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

use tokio::time::Instant;

#[derive(Clone)]
pub struct MobWzLife {
    pub wz: i32,
    pub x: i16,
    pub y: i16,
    pub fh: u16,
    pub mob_time: u64,
}

#[derive(Clone)]
pub struct MobWzInfo {
    pub mad: i16,
    pub mdd: i16,
    pub pad: i16,
    pub pdd: i16,
    pub acc: i16,
    pub body_attack: i8,
    pub exp: i32,
    pub eva: i16,
    pub fs: f32,
    pub level: i16,
    pub max_hp: i32,
    pub max_mp: i32,
    pub mob_type: i16,
    pub pushed: i8,
    pub speed: i16,
    pub summon_type: i16,
    pub undead: i8,
}

#[derive(Clone)]
pub struct MobModel {
    pub id: u32,
    pub pos_x: i16,
    pub pos_y: i16,
    pub last_x: i16,
    pub last_y: i16,
    pub fh: u16,
    pub new_state: u8,
    pub hp: i32,
    pub mp: i32,
}

#[derive(Clone)]
pub enum LifeState {
    Alive,
    Dead(DeathState),
}

#[derive(Clone)]
pub struct DeathState {
    pub died_at: Instant,
}

#[derive(Clone)]
pub struct MobMovement {
    pub command: u8,
    pub x: i16,
    pub y: i16,
    pub last_x: i16,
    pub last_y: i16,
    pub fh: u16,
    pub new_state: u8,
    pub duration: i16,
}
