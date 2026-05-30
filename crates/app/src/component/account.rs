/* app/src/component/account.rs
 * The purpose of this module is to provide an account component.
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

#[derive(Component)]
pub struct MapleAccount {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub pin: Option<String>,
    pub pic: Option<String>,
    pub gender_wz: i16,
    pub accepted_tos: bool,
    pub banned: bool,
    pub admin: bool,
}
