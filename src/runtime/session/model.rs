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

#[derive(Clone)]
pub struct Session {
    pub id: i32,
    pub acc: Option<Account>,
    pub channel: Option<Channel>,
    pub char: Option<Character>,
    pub hwid: Option<String>,
    pub map: Option<Map>,
    pub tx: UnboundedSender<Packet>,
    pub world: Option<World>,
    pub playing: bool,
}

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

pub struct SessionData {
    sessions: HashMap<i32, Session>,
    by_world: HashMap<i32, HashSet<i32>>,
    by_channel: HashMap<i32, HashSet<i32>>,
    by_map: HashMap<i32, HashSet<i32>>,
    by_map_channel: HashMap<(i32, i32), HashSet<i32>>,
    by_channel_world: HashMap<(i32, i32), HashSet<i32>>,
}

pub struct SessionStore {
    data: RwLock<SessionData>,
    pub next_id: AtomicI32,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            next_id: AtomicI32::new(1),
        }
    }

    pub fn get_by_map_channel_world(&self, wz_id: i32, channel_id: i32) -> Vec<Session>
    pub fn get_by_map_world(&self, wz_id: i32, world_id: i32) -> Vec<Session>
    pub fn get_by_map(&self, wz_id: i32) -> Vec<Session>
    pub fn get_by_channel_world(&self, channel_id: i32, world_id: i32) -> Vec<Session>
    pub fn get_by_channel(&self, channel_id: i32) -> Vec<Session>
    pub fn get_by_world(&self, world_id: i32) -> Vec<Session>
    pub fn get_all(&self) -> Vec<Session>

    pub fn insert(&self, mut session: Session) -> i32 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        session.id = id;
        self.sessions
            .write()
            .expect("session store write lock poisoned")
            .insert(id, session);
        id
    }

    pub fn get(&self, id: i32) -> Option<Session> {
        self.sessions
            .read()
            .expect("session store read lock poisoned")
            .get(&id)
            .cloned()
    }

    pub fn update(&self, id: i32, f: impl FnOnce(&mut Session)) {
        let mut guard = self
            .sessions
            .write()
            .expect("session store write lock poisoned");
        if let Some(session) = guard.get_mut(&id) {
            f(session);
        }
    }

    pub fn remove(&self, id: i32) {
        self.sessions
            .write()
            .expect("session store write lock poisoned")
            .remove(&id);
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}
