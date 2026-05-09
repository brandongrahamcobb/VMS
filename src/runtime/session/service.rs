use crate::models::account::model::Account;
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::models::shroom::map::model::Map;
use crate::models::shroom::world::model::World;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use core::sync::atomic::AtomicI32;
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc::UnboundedSender;

impl Session {
    pub fn get_acc(&self) -> Result<Account, SessionError> {
        if let Some(acc) = self.acc.clone() {
            return Ok(acc);
        } else {
            return Err(SessionError::NoAccount(self.id));
        }
    }

    pub fn get_channel(&self) -> Result<Channel, SessionError> {
        if let Some(channel) = self.channel.clone() {
            return Ok(channel);
        } else {
            return Err(SessionError::NoChannel(self.id));
        }
    }

    pub fn get_char(&self) -> Result<Character, SessionError> {
        if let Some(char) = self.char.clone() {
            return Ok(char);
        } else {
            return Err(SessionError::NoChar(self.id));
        }
    }

    pub fn get_map(&self) -> Result<Map, SessionError> {
        if let Some(map) = self.map.clone() {
            return Ok(map);
        } else {
            return Err(SessionError::NoMap(self.id));
        }
    }

    pub fn get_world(&self) -> Result<World, SessionError> {
        if let Some(world) = self.world.clone() {
            return Ok(world);
        } else {
            return Err(SessionError::NoWorld(self.id));
        }
    }
}

impl SessionData {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            by_world: HashMap::new(),
            by_channel: HashMap::new(),
            by_map: HashMap::new(),
            by_map_channel: HashMap::new(),
            by_channel_world: HashMap::new(),
        }
    }

    fn reindex(
        &mut self,
        id: i32,
        old_world: Option<i32>,
        old_channel: Option<i32>,
        old_map: Option<i32>,
        new_world: Option<i32>,
        new_channel: Option<i32>,
        new_map: Option<i32>,
    ) {
        if old_world != new_world {
            if let Some(w) = old_world {
                self.by_world.entry(w).or_default().remove(&id);
            }
            if let Some(w) = new_world {
                self.by_world.entry(w).or_default().insert(id);
            }
        }
        if old_channel != new_channel {
            if let Some(c) = old_channel {
                self.by_channel.entry(c).or_default().remove(&id);
            }
            if let Some(c) = new_channel {
                self.by_channel.entry(c).or_default().insert(id);
            }
        }
        if old_map != new_map {
            if let Some(m) = old_map {
                self.by_map.entry(m).or_default().remove(&id);
            }
            if let Some(m) = new_map {
                self.by_map.entry(m).or_default().insert(id);
            }
        }
        let old_cw = old_channel.zip(old_world);
        let new_cw = new_channel.zip(new_world);
        if old_cw != new_cw {
            if let Some(key) = old_cw {
                self.by_channel_world.entry(key).or_default().remove(&id);
            }
            if let Some(key) = new_cw {
                self.by_channel_world.entry(key).or_default().insert(id);
            }
        }
        let old_mc = old_map.zip(old_channel);
        let new_mc = new_map.zip(new_channel);
        if old_mc != new_mc {
            if let Some(key) = old_mc {
                self.by_map_channel.entry(key).or_default().remove(&id);
            }
            if let Some(key) = new_mc {
                self.by_map_channel.entry(key).or_default().insert(id);
            }
        }
    }

    fn resolve(&self, ids: &HashSet<i32>, exclude: i32) -> Vec<Session> {
        ids.iter()
            .filter(|&&id| id != exclude)
            .filter_map(|id| self.sessions.get(id).cloned())
            .collect()
    }

    fn resolve_intersection(
        &self,
        a: &HashSet<i32>,
        b: &HashSet<i32>,
        exclude: i32,
    ) -> Vec<Session> {
        a.intersection(b)
            .filter(|&&id| id != exclude)
            .filter_map(|id| self.sessions.get(id).cloned())
            .collect()
    }
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(SessionData::new()),
            next_id: AtomicI32::new(1),
        }
    }

    pub fn insert(&self, mut session: Session) -> i32 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        session.id = id;
        self.data
            .write()
            .expect("poisoned")
            .sessions
            .insert(id, session);
        id
    }

    pub fn get(&self, id: i32) -> Option<Session> {
        self.data
            .read()
            .expect("poisoned")
            .sessions
            .get(&id)
            .cloned()
    }

    pub fn update(&self, id: i32, f: impl FnOnce(&mut Session)) {
        let mut data = self.data.write().expect("poisoned");
        if let Some(session) = data.sessions.get_mut(&id) {
            let old_world = session.world.as_ref().map(|w| w.model.id);
            let old_channel = session.channel.as_ref().map(|c| c.model.id);
            let old_map = session.map.as_ref().map(|m| m.model.wz_id);
            f(session);
            let new_world = session.world.as_ref().map(|w| w.model.id);
            let new_channel = session.channel.as_ref().map(|c| c.model.id);
            let new_map = session.map.as_ref().map(|m| m.model.wz_id);
            data.reindex(
                id,
                old_world,
                old_channel,
                old_map,
                new_world,
                new_channel,
                new_map,
            );
        }
    }

    pub fn remove(&self, id: i32) {
        let mut data = self.data.write().expect("poisoned");
        if let Some(s) = data.sessions.remove(&id) {
            data.reindex(
                id,
                s.world.as_ref().map(|w| w.model.id),
                s.channel.as_ref().map(|c| c.model.id),
                s.map.as_ref().map(|m| m.model.wz_id),
                None,
                None,
                None,
            );
        }
    }

    pub fn get_by_map_channel_world(
        &self,
        wz_id: i32,
        channel_id: i32,
        world_id: i32,
        exclude: i32,
    ) -> Vec<Session> {
        let data = self.data.read().expect("poisoned");
        let mc = data
            .by_map_channel
            .get(&(wz_id, channel_id))
            .cloned()
            .unwrap_or_default();
        let w = data.by_world.get(&world_id).cloned().unwrap_or_default();
        data.resolve_intersection(&mc, &w, exclude)
    }

    pub fn get_by_map_world(&self, wz_id: i32, world_id: i32, exclude: i32) -> Vec<Session> {
        let data = self.data.read().expect("poisoned");
        let m = data.by_map.get(&wz_id).cloned().unwrap_or_default();
        let w = data.by_world.get(&world_id).cloned().unwrap_or_default();
        data.resolve_intersection(&m, &w, exclude)
    }

    pub fn get_by_map(&self, wz_id: i32, exclude: i32) -> Vec<Session> {
        let data = self.data.read().expect("poisoned");
        let empty = HashSet::new();
        let ids = data.by_map.get(&wz_id).unwrap_or(&empty);
        data.resolve(ids, exclude)
    }

    pub fn get_by_channel_world(
        &self,
        channel_id: i32,
        world_id: i32,
        exclude: i32,
    ) -> Vec<Session> {
        let data = self.data.read().expect("poisoned");
        let empty = HashSet::new();
        let ids = data
            .by_channel_world
            .get(&(channel_id, world_id))
            .unwrap_or(&empty);
        data.resolve(ids, exclude)
    }

    pub fn get_by_channel(&self, channel_id: i32, exclude: i32) -> Vec<Session> {
        let data = self.data.read().expect("poisoned");
        let empty = HashSet::new();
        let ids = data.by_channel.get(&channel_id).unwrap_or(&empty);
        data.resolve(ids, exclude)
    }

    pub fn get_by_world(&self, world_id: i32, exclude: i32) -> Vec<Session> {
        let data = self.data.read().expect("poisoned");
        let empty = HashSet::new();
        let ids = data.by_world.get(&world_id).unwrap_or(&empty);
        data.resolve(ids, exclude)
    }

    pub fn get_all(&self, exclude: i32) -> Vec<Session> {
        let data = self.data.read().expect("poisoned");
        data.sessions
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
