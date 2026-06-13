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

use crate::component::session::Transitioning;
use crate::resource::custom_resource::ClientMap;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Query, ResMut};

pub fn cleanup_stale_transitions(
    mut commands: Commands,
    mut client_map: ResMut<ClientMap>,
    query: Query<(Entity, &Transitioning)>,
) {
    for (entity, transitioning) in query.iter() {
        if transitioning.started_at.elapsed() > std::time::Duration::from_secs(10) {
            tracing::debug!("Stale transition, despawning {:?}", entity);
            client_map.0.retain(|_, v| *v != entity);
            commands.entity(entity).despawn();
        }
    }
}
