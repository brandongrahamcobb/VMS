/* app/src/system/startup.rs
 * The purpose of this module is to provide a system for startup.
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
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use config::settings;

use crate::component;

pub fn spawn_worlds(mut commands: Commands) {
    for world_component in component::world::WORLDS {
        let world_entity = commands.spawn(world_component).id();
        let channels = spawn_channels(commands, world_component.base_port, world_entity);
    }
}

fn spawn_channels(mut commands: Commands, base_port: i16, world_entity: Entity) {
    let capacity = settings::get_channel_capacity()?;
    let count: i32 = settings::get_channel_count()?;
    let flag: i16 = settings::get_channel_flag()?;
    let first_port: i16 = base_port + 1;
    for offset in 0..count {
        let port: i16 = (first_port + offset) as i16;
        commands.spawn((
            MapleChannel {
                capacity,
                flag,
                port,
            },
            ChildOf(world_entity),
        ));
    }
}
