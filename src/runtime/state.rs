use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::db::pool::DbPool;
use crate::runtime::error::RuntimeError;
use crate::runtime::session::SessionStore;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct State {
    pub db: DbPool,
    pub sessions: SessionStore,
    pub map_index: HashMap<(i8, i8, i32), HashSet<i32>>,
}

pub type SharedState = Arc<Mutex<State>>;

impl State {
    pub fn new() -> Result<Self, RuntimeError> {
        let db_url = settings::get_db_url()?;
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let db = Pool::builder()
            .build(manager)
            .map_err(|_| RuntimeError::from(DatabaseError::DatabaseConnectionError))?;
        let sessions = SessionStore::new();
        let map_index = HashMap::new();
        let shared_state = State {
            db,
            sessions,
            map_index,
        };
        Ok(shared_state)
    }

    pub fn set_location(&mut self, session_id: &i32, world_id: &i8, channel_id: &i8, map_id: &i32) {
        let key = (*world_id, *channel_id, *map_id);
        self.map_index.entry(key).or_default().insert(*session_id);
    }
}
