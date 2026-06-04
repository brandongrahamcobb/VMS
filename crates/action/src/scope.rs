/* scope.rs
 * The purpose of this module is to provide the scope a relay action can affect.
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

#[derive(Clone)]
pub enum ActionScope {
    Map(MapScope),
    Channel(ChannelScope),
    World,
    Local,
    Global,
}

#[derive(Clone)]
pub enum MapScope {
    SameChannelSameWorld,
    AllChannelsSameWorld,
    AllChannelsAllWorlds,
}

#[derive(Clone)]
pub enum ChannelScope {
    SameWorld,
    AllWorlds,
}

#[derive(Clone)]
pub enum TickScope {
    Global,
    World {
        world_id: i16,
    },
    Channel {
        world_id: i16,
        channel_id: u8,
    },
    Map {
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
    },
}
