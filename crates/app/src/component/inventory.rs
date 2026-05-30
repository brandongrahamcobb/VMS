/* app/src/component/inventory.rs
 * The purpose of this module is to provide a inventory component.
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
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use config::settings;

#[derive(Component)]
pub struct MapleInventory {
    pub equip_tab_capacity: i8,
    pub use_tab_capacity: i8,
    pub etc_tab_capacity: i8,
    pub setup_tab_capacity: i8,
    pub cash_tab_capacity: i8,
}

fn spawn_inventory(mut commands: Commands, character_entity: Entity) {
    let capacity = settings::get_default_inventory_capacity()?;
    commands.spawn((
        MapleInventory {
            equip_tab_capacity: capacity,
            use_tab_capacity: capacity,
            etc_tab_capacity: capacity,
            setup_tab_capacity: capacity,
            cash_tab_capacity: capacity,
        },
        ChildOf(character_entity),
    ));
}
