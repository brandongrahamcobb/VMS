use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::db::pool::DbPool;
use crate::runtime::error::{RuntimeError, SessionError};
use crate::runtime::session::{Session, SessionStore};
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

    pub async fn create_location(
        &mut self,
        session: &Session,
        world_id: &i8,
        channel_id: &i8,
        map_id: &i32,
    ) -> Result<(), RuntimeError> {
        let key = (*world_id, *channel_id, *map_id);
        self.map_index.entry(key).or_default().insert(session.id);
    }

    pub async fn set_location(
        &mut self,
        session: &Session,
        world_id: &Option<i8>,
        channel_id: &Option<i8>,
        map_id: &Option<i32>,
    ) -> Result<(), RuntimeError> {
        let key = (
            session.world_id.unwrap(),
            session.channel_id.unwrap(),
            session.map_id.unwrap(),
        );
        self.map_index.remove(&key);
        let session = {
            if let Some(w) = world_id {
                self.sessions.update(&session.id, |s| s.world_id = Some(*w));
            }
            if let Some(c) = channel_id {
                self.sessions
                    .update(&session.id, |s| s.channel_id = Some(*c));
            }
            if let Some(m) = map_id {
                self.sessions.update(&session.id, |s| s.map_id = Some(*m));
            }
            self.sessions
                .get(&session.id)
                .ok_or(SessionError::NotFound(session.id))?
        };
        let key = (
            session.world_id.unwrap(),
            session.channel_id.unwrap(),
            session.map_id.unwrap(),
        );
        self.map_index.entry(key).or_default().insert(session.id);
        Ok(())
    }
}
