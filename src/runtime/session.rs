use crate::models::account::model::Account;
use crate::models::channel::model::Channel;
use crate::models::character::model::Character;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::net::packet::model::Packet;
use core::sync::atomic::AtomicI32;
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct Session {
    pub id: i32,
    pub acc: Account,
    pub authenticated: bool,
    pub channel: Channel,
    pub char: Character,
    pub hwid: String,
    pub map: Map,
    pub playing: bool,
    pub tx: UnboundedSender<Packet>,
    pub world: World,
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
