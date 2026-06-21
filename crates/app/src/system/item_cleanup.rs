/* app/src/system/transition_cleanup.rs
 * The purpose of this module is to provide a system for cleaning up transitions.
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

use crate::component::item::{Lootable, MapleItem};
use crate::message::result::TickResult;
use crate::system::packet::tick::result::remove_loot_result;
use crate::system::system_params::LocationParams;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::{Commands, Query};

pub fn cleanup_stale_items(
    mut commands: Commands,
    loc_params: LocationParams,
    items: Query<(Entity, &MapleItem, &ChildOf)>,
    lootable: Query<(&Lootable, &ChildOf)>,
    mut results: MessageWriter<TickResult>,
) {
    for (lootable, parent) in lootable.iter() {
        let Ok((entity, item, parent)) = items.get(parent.0) else {
            continue;
        };
        if lootable.dropped_at.elapsed() > std::time::Duration::from_secs(30) {
            tracing::debug!("Stale item, despawning {:?}", entity);
            commands.entity(entity).despawn();
        }
        let Ok((_, map, parent)) = loc_params.maps.get(parent.0) else {
            continue;
        };
        let Ok((_, channel, parent)) = loc_params.channels.get(parent.0) else {
            continue;
        };
        let Ok((_, world)) = loc_params.worlds.get(parent.0) else {
            continue;
        };
        remove_loot_result::write_result(
            item.id,
            world.base.id,
            channel.id,
            map.base.wz,
            &mut results,
        );
    }
} //delete from db
