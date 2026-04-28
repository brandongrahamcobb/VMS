use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::db::pool::DbPool;
use crate::runtime::error::RuntimeError;
use crate::runtime::session::SessionStore;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct State {
    pub db: DbPool,
    pub sessions: SessionStore,
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
        let shared_state = State { db, sessions };
        Ok(shared_state)
    }
}
