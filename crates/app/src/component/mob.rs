/* app/src/component/mob.rs
 * The purpose of this module is to provide a mob component.
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

use base::mob::BaseMob;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Component)]
pub struct MapleMob {
    pub id: u32,
    pub new_state: u8,
    pub died_at: Instant,
    pub dead: bool,
    pub base: BaseMob,
}

#[derive(Clone, Component, Default)]
pub struct MobIndex {
    pub counter: u32,
    pub map: HashMap<u32, Entity>,
}

impl MobIndex {
    pub fn next_id(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}
