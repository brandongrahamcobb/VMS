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

use base::map::BaseMap;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::Commands;

use crate::component::map::MapleMap;
use crate::component::mob::MobIndex;

pub fn init(
    commands: &mut Commands,
    base_map: BaseMap,
    mob_index: &MobIndex,
    channel_entity_parent: Entity,
) -> Entity {
    let map: MapleMap = MapleMap {
        vacant: false,
        base: base_map.clone(),
    };
    commands
        .spawn((
            map.clone(),
            mob_index.clone(),
            ChildOf(channel_entity_parent),
        ))
        .id()
}
