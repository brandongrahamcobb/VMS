/* app/src/system/lazy.rs
 * The purpose of this module is to provide a system for lazy startup.
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

fn lazy_spawn_mobs(mut commands: Commands, map_entity: Entity) {}

fn lazy_spawn_maps(mut commands: Commands, channel_entity: Entity) {}

fn lazy_spawn_jobs(mut commands: Commands) {}

fn lazy_spawn_portals(mut commands: Command, map_entity: Entity) {}

fn lazy_spawn_skills(mut commands: Commands) {}
