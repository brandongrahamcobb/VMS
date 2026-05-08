use crate::models::account::model::Account;
use crate::models::channel::model::Channel;
use crate::models::character::model::Character;
use crate::models::map::model::Map;
use crate::models::world::model::World;
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

pub struct SessionStore {
    pub sessions: RwLock<HashMap<i32, Session>>,
    pub next_id: AtomicI32,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            next_id: AtomicI32::new(1),
        }
    }

    pub fn iter(&self) -> Vec<Session> {
        self.sessions
            .read()
            .expect("session store poisoned")
            .values()
            .cloned()
            .collect()
    }

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
