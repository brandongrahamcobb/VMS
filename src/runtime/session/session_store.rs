/* session_store.rs
 * The purpose of this module is to provide the async store for sessions.
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

use crate::runtime::session::model::Session;
use core::sync::atomic::AtomicI32;
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::Ordering;

pub struct SessionStore {
    pub next_id: AtomicI32,
    pub sessions: RwLock<HashMap<i32, Session>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            next_id: AtomicI32::new(1),
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, mut session: Session) -> i32 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        session.id = id;
        self.sessions.write().expect("poisoned").insert(id, session);
        id
    }

    pub fn get(&self, id: i32) -> Option<Session> {
        self.sessions.read().expect("poisoned").get(&id).cloned()
    }

    pub fn update(&self, id: i32, f: impl FnOnce(&mut Session)) {
        let mut sessions = self.sessions.write().expect("poisoned");
        if let Some(session) = sessions.get_mut(&id) {
            f(session);
        }
    }

    pub fn remove(&self, id: i32) {
        self.sessions.write().expect("poisoned").remove(&id);
    }

    pub fn get_by_channel(&self, channel_id: u8, exclude: i32) -> Vec<Session> {
        self.sessions
            .read()
            .expect("poisoned")
            .values()
            .filter(|s| s.id != exclude && s.channel_id == Some(channel_id))
            .cloned()
            .collect()
    }

    pub fn get_by_channel_world(&self, channel_id: u8, world_id: i16, exclude: i32) -> Vec<Session> {
        self.sessions
            .read()
            .expect("poisoned")
            .values()
            .filter(|s| {
                s.id != exclude && s.channel_id == Some(channel_id) && s.world_id == Some(world_id)
            })
            .cloned()
            .collect()
    }

    pub fn get_by_map(&self, map_wz: i32, exclude: i32) -> Vec<Session> {
        self.sessions
            .read()
            .expect("poisoned")
            .values()
            .filter(|s| s.id != exclude && s.map_wz == Some(map_wz))
            .cloned()
            .collect()
    }

    pub fn get_by_map_channel_world(
        &self,
        map_wz: i32,
        channel_id: u8,
        world_id: i16,
        exclude: i32,
    ) -> Vec<Session> {
        self.sessions
            .read()
            .expect("poisoned")
            .values()
            .filter(|s| {
                s.id != exclude
                    && s.map_wz == Some(map_wz)
                    && s.channel_id == Some(channel_id)
                    && s.world_id == Some(world_id)
            })
            .cloned()
            .collect()
    }

    pub fn get_by_map_world(&self, map_wz: i32, world_id: i16, exclude: i32) -> Vec<Session> {
        self.sessions
            .read()
            .expect("poisoned")
            .values()
            .filter(|s| s.id != exclude && s.map_wz == Some(map_wz) && s.world_id == Some(world_id))
            .cloned()
            .collect()
    }

    pub fn get_by_world(&self, world_id: i16, exclude: i32) -> Vec<Session> {
        self.sessions
            .read()
            .expect("poisoned")
            .values()
            .filter(|s| s.id != exclude && s.world_id == Some(world_id))
            .cloned()
            .collect()
    }

    pub fn get_all(&self, exclude: i32) -> Vec<Session> {
        self.sessions
            .read()
            .expect("poisoned")
            .values()
            .filter(|s| s.id != exclude)
            .cloned()
            .collect()
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}
