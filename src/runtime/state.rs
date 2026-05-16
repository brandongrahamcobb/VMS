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

    pub async fn get_world(&self, world_id: i16) -> Result<World, StateError> {
        self.worlds
            .read()
            .await
            .get(&world_id)
            .ok_or(StateError::NoWorld(world_id))
            .cloned()
    }

    pub async fn get_channel(&self, world_id: i16, channel_id: u8) -> Result<Channel, StateError> {
        self.get_world(world_id)
            .await?
            .channels
            .get(&channel_id)
            .ok_or(StateError::NoChannel(channel_id))
            .cloned()
    }

    pub async fn get_map(
        &self,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
    ) -> Result<Map, StateError> {
        self.get_channel(world_id, channel_id)
            .await?
            .maps
            .get(&map_wz)
            .ok_or(StateError::NoMap(map_wz))
            .cloned()
    }

    pub async fn insert_map(
        &self,
        world_id: i16,
        channel_id: u8,
        map: Map,
    ) -> Result<(), StateError> {
        self.worlds
            .write()
            .await
            .get_mut(&world_id)
            .ok_or(StateError::NoWorld(world_id))?
            .channels
            .get_mut(&channel_id)
            .ok_or(StateError::NoChannel(channel_id))?
            .maps
            .insert(map.model.wz, map);
        Ok(())
    }

    pub async fn delete_map(
        &self,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
    ) -> Result<(), StateError> {
        self.worlds
            .write()
            .await
            .get_mut(&world_id)
            .ok_or(StateError::NoWorld(world_id))?
            .channels
            .get_mut(&channel_id)
            .ok_or(StateError::NoChannel(channel_id))?
            .maps
            .remove(&map_wz);
        Ok(())
    }
}
