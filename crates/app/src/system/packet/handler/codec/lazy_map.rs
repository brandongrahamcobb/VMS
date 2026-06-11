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
use base::mob::BaseMob;
use base::portal::BasePortal;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;

use crate::component::mob::{MapleMob, MobIndex};
use crate::component::npc::MapleNpc;
use crate::system::packet::handler::codec::{init_map, init_mobs, init_portals};

pub fn lazy_load_map(
    commands: &mut Commands,
    channel_entity: Entity,
    base_map: BaseMap,
    base_portals: Vec<BasePortal>,
    base_mobs: Vec<BaseMob>,
) -> (Entity, Vec<MapleMob>, Vec<MapleNpc>) {
    let mut mob_index = MobIndex::default();
    let map_entity = init_map::init(commands, base_map, &mob_index, channel_entity);
    init_portals::init(commands, base_portals, map_entity.clone());
    let mobs = init_mobs::init(commands, base_mobs, &mut mob_index, map_entity);
    let npcs = Vec::new(); // TODO: Spawn NPCS
    (map_entity, mobs, npcs)
}
