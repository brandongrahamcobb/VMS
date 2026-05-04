use core::sync::atomic::AtomicI32;
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc::UnboundedSender;

use crate::net::packet::packet::Packet;

#[derive(Clone)]
pub enum SessionState {
    BeforeLogin,
    AfterLogin,
    Transition,
    InGame,
}

#[derive(Clone)]
pub struct Session {
    pub id: i32,
    pub acc_id: i32,
    pub authenticated: bool,
    pub hwid: String,
    pub world_id: Option<i8>,
    pub channel_id: Option<i8>,
    pub map_id: Option<i32>,
    pub char_id: Option<i32>,
    pub tx: UnboundedSender<Packet>,
    pub playing: bool,
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

    pub fn insert(&self, mut session: Session) -> i32 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        session.id = id;
        self.sessions
            .write()
            .expect("session store write lock poisoned")
            .insert(id, session);
        id
    }

    pub fn get(&self, id: &i32) -> Option<Session> {
        self.sessions
            .read()
            .expect("session store read lock poisoned")
            .get(id)
            .cloned()
    }

    pub fn get_by_acc_id(&self, acc_id: i32) -> Option<Session> {
        self.sessions
            .read()
            .expect("session store read lock poisoned")
            .values()
            .find(|s| s.acc_id == acc_id)
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
