/* app/src/resource/custom_resource.rs
 * The purpose of this module is to define custom resources.
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
use std::sync::{Mutex, mpsc::Receiver};

use bevy::ecs::entity::Entity;
use db::pool::DbPool;
use session::session_store::SessionStore;

#[derive(Resource)]
pub struct CustomReceiver(Mutex<Receiver<CustomPluginEvent>>);

#[derive(Resource)]
pub struct CustomSender(pub Mutex<Sender<CustomPluginCommand>>);

#[derive(Resource)]
pub struct Pool(DbPool);

#[derive(Resource)]
pub struct ClientMap(pub HashMap<i32, Entity>)
