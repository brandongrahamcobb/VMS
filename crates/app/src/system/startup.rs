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
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::Commands;
use config::settings;

use crate::component::channel::MapleChannel;
use crate::component::world::MapleWorld;

pub fn spawn_worlds(mut commands: Commands) {
    for base_world in base::world::WORLDS {
        let world: MapleWorld = MapleWorld { base: *base_world };
        let world_entity = commands.spawn(world.clone()).id();
        spawn_channels(&mut commands, world.base.base_port, world_entity);
    }
}

fn spawn_channels(commands: &mut Commands, base_port: i16, world_entity: Entity) {
    let ports = inc::channel::get_channel_ports(base_port);
    for (id, port) in ports.iter().enumerate() {
        let Ok(capacity) = settings::get_channel_capacity() else {
            continue;
        };
        let Ok(flag) = settings::get_channel_flag() else {
            continue;
        };
        commands.spawn((
            MapleChannel {
                id: id as u8,
                capacity,
                flag,
                port: *port,
            },
            ChildOf(world_entity),
        ));
    }
}
