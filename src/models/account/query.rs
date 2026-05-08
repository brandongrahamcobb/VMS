use crate::db::schema::accounts;
use crate::models::account::model::AccountModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SaveChangesDsl};

pub async fn get_account_model_by_username(
    state: &SharedState,
    user: String,
) -> QueryResult<AccountModel> {
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
        .first::<AccountModel>(&mut conn)
}

pub async fn get_account_model_by_id(
    state: &SharedState,
    acc_id: i32,
) -> QueryResult<AccountModel> {
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
        .filter(accounts::id.eq(acc_id.clone()))
        .first::<AccountModel>(&mut conn)
}

pub async fn update_by_model(state: &SharedState, acc: AccountModel) -> QueryResult<AccountModel> {
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
