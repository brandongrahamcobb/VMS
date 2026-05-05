use crate::db::schema::{accounts, characters};
use crate::models::account::model::Account;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SaveChangesDsl};

pub async fn get_account_by_username(state: &SharedState, user: &str) -> QueryResult<Account> {
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

pub async fn get_account_by_id(state: &SharedState, id: &i32) -> QueryResult<Account> {
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
        .filter(accounts::id.eq(*id as i32))
        .first::<Account>(&mut conn)
}

pub async fn get_account_by_char_id(state: &SharedState, char_id: &i32) -> QueryResult<Account> {
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
    let acc_id = characters::table
        .filter(&characters::id.eq(&char_id))
        .select(&characters::acc_id)
        .first::<i32>(&mut conn)?;
    accounts::table
        .filter(accounts::id.eq(acc_id))
        .first::<Account>(&mut conn)
}

pub async fn update(state: &SharedState, acc: &Account) -> QueryResult<Account> {
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

pub async fn get_session_id_by_acc_id(
    state: &SharedState,
    acc_id: &i32,
) -> QueryResult<Option<i32>> {
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
        .filter(accounts::id.eq(acc_id))
        .select(accounts::session_id)
        .first::<Option<i32>>(&mut conn)
}
