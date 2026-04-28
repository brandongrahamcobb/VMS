use crate::models::account::model::Account;
use crate::db::schema::accounts;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SaveChangesDsl};

pub async fn get_account_by_username(state: SharedState, user: &str) -> QueryResult<Account> {
    let db = {
        let state = state.lock().await;
        state.db.clone()
    };
    let mut conn = db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    accounts::table
        .filter(accounts::username.eq(&user))
        .first::<Account>(&mut conn)
}

pub async fn get_account_by_id(state: SharedState, id: i64) -> QueryResult<Account> {
    let db = {
        let state = state.lock().await;
        state.db.clone()
    };
    let mut conn = db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    accounts::table
        .filter(accounts::id.eq(id as i64))
        .first::<Account>(&mut conn)
}

pub async fn update(state: SharedState, acc: &Account) -> QueryResult<Account> {
    let db = {
        let state = state.lock().await;
        state.db.clone()
    };
    let mut conn = db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    acc.save_changes(&mut conn)
}
