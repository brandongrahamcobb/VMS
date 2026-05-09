use crate::models::account::model::Account;
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::models::shroom::map::model::Map;
use crate::models::shroom::world::model::World;
use crate::net::packet::model::Packet;
use core::sync::atomic::AtomicI32;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
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

pub struct SessionData {
    pub sessions: HashMap<i32, Session>,
    pub by_world: HashMap<i16, HashSet<i32>>,
    pub by_channel: HashMap<i16, HashSet<i32>>,
    pub by_map: HashMap<i32, HashSet<i32>>,
    pub by_map_channel: HashMap<(i32, i16), HashSet<i32>>,
    pub by_channel_world: HashMap<(i16, i16), HashSet<i32>>,
}

pub struct SessionStore {
    pub data: RwLock<SessionData>,
    pub next_id: AtomicI32,
}
