/* app/src/system/handler/codec/init_mobs.rs
 * The purpose of this module is to process mobs initialization.
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

use std::time::Instant;

use base::mob::BaseMob;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::Commands;

use crate::component::hp::MapleHealth;
use crate::component::mob::{MapleMob, MobIndex};
use crate::component::mp::MapleMana;
use crate::component::position::{MapleCurrentPosition, MapleLastPosition};

pub fn init(
    commands: &mut Commands,
    base_mobs: Vec<BaseMob>,
    mob_index: &mut MobIndex,
    map_entity_parent: Entity,
) -> Vec<MapleMob> {
    let mut mobs: Vec<MapleMob> = Vec::new();
    for base_mob in base_mobs.clone() {
        let mob: MapleMob = MapleMob {
            id: mob_index.next_id(),
            new_state: 0,
            died_at: Instant::now(),
            dead: false,
            base: base_mob,
        };
        mobs.push(mob.clone());
        let mob_entity = commands.spawn((mob, ChildOf(map_entity_parent))).id();
        let curr_pos = MapleCurrentPosition {
            x: 0,
            y: 0,
            fh: None,
        };
        commands.spawn((curr_pos, ChildOf(mob_entity)));
        let last_pos = MapleLastPosition { x: 0, y: 0 };
        commands.spawn((last_pos, ChildOf(mob_entity)));
        let hp = MapleHealth {
            amount: base_mob.max_hp as i32,
            max: base_mob.max_hp as i32,
        };
        commands.spawn((hp, ChildOf(mob_entity)));
        let mp = MapleMana {
            amount: base_mob.max_mp,
            max: base_mob.max_mp,
        };
        commands.spawn((mp, ChildOf(mob_entity)));
    }
    mobs
}
