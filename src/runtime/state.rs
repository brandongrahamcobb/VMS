/* state.rs
 * The purpose of this module is to provide a shared state across the program.
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

use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::db::pool::DbPool;
use crate::models::channel::wrapper::Channel;
use crate::models::map::wrapper::Map;
use crate::models::world;
use crate::models::world::wrapper::World;
use crate::runtime::error::StateError;
use crate::runtime::session::session_store::SessionStore;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub struct State {
    pub db: DbPool,
    pub sessions: SessionStore,
    pub worlds: Arc<RwLock<HashMap<i16, World>>>,
}

pub type SharedState = Arc<Mutex<State>>;

impl State {
    pub fn new() -> Result<Self, StateError> {
        let db_url = settings::get_db_url()?;
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let db = Pool::builder()
            .build(manager)
            .map_err(|_| StateError::from(DatabaseError::DatabaseConnectionError))?;
        let sessions = SessionStore::new();
        let worlds = world::service::load_worlds()?;
        let shared_state = State {
            db,
            sessions,
            worlds: Arc::new(RwLock::new(worlds)),
        };
        Ok(shared_state)
    }

    pub async fn with_world<F, R>(&self, world_id: i16, f: F) -> Result<R, StateError>
    where
        F: FnOnce(&World) -> R,
    {
        let worlds = self.worlds.read().await;
        let world = worlds.get(&world_id).ok_or(StateError::NoWorld(world_id))?;
        Ok(f(world))
    }

    pub async fn with_channel<F, R>(
        &self,
        world_id: i16,
        channel_id: u8,
        f: F,
    ) -> Result<R, StateError>
    where
        F: FnOnce(&Channel) -> R,
    {
        self.with_world(world_id, |world| {
            world
                .channels
                .get(&channel_id)
                .map(f)
                .ok_or(StateError::NoChannel(channel_id))
        })
        .await?
    }

    pub async fn with_mut_channel<F, R>(
        &self,
        world_id: i16,
        channel_id: u8,
        f: F,
    ) -> Result<R, StateError>
    where
        F: FnOnce(&mut Channel) -> R,
    {
        let mut worlds = self.worlds.write().await;
        let world = worlds
            .get_mut(&world_id)
            .ok_or(StateError::NoWorld(world_id))?;
        let channel = world
            .channels
            .get_mut(&channel_id)
            .ok_or(StateError::NoChannel(channel_id))?;
        Ok(f(channel))
    }

    pub async fn with_map<F, R>(
        &self,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
        f: F,
    ) -> Result<R, StateError>
    where
        F: FnOnce(&Map) -> R,
    {
        self.with_channel(world_id, channel_id, |channel| {
            channel
                .maps
                .get(&map_wz)
                .map(f)
                .ok_or(StateError::NoMap(map_wz))
        })
        .await?
    }

    pub async fn with_mut_map<F, R>(
        &self,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
        f: F,
    ) -> Result<R, StateError>
    where
        F: FnOnce(&mut Map) -> R,
    {
        let mut worlds = self.worlds.write().await;
        let world = worlds
            .get_mut(&world_id)
            .ok_or(StateError::NoWorld(world_id))?;
        let channel = world
            .channels
            .get_mut(&channel_id)
            .ok_or(StateError::NoChannel(channel_id))?;
        let map = channel
            .maps
            .get_mut(&map_wz)
            .ok_or(StateError::NoMap(map_wz))?;
        Ok(f(map))
    }
}
