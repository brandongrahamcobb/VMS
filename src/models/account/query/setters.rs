use crate::db::schema::accounts;
use crate::models::account::model::AccountModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn set_pic_by_account_id(
    state: &SharedState,
    acc_id: i32,
    pic: String,
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
    diesel::update(accounts::table.filter(accounts::id.eq(acc_id)))
        .set(accounts::pic.eq(pic))
        .get_result::<AccountModel>(&mut conn)
}

pub async fn accept_tos_by_account_id(
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
    diesel::update(accounts::table.filter(accounts::id.eq(acc_id)))
        .set(accounts::accepted_tos.eq(true))
        .get_result::<AccountModel>(&mut conn)
}

pub async fn update_accounts(
    state: &SharedState,
    acc_models: Vec<AccountModel>,
) -> QueryResult<Vec<AccountModel>> {
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
    let mut results = Vec::new();
    for acc_model in &acc_models {
        results.push(
            diesel::insert_into(accounts::table)
                .values(acc_model)
                .on_conflict(accounts::id)
                .do_update()
                .set(acc_model)
                .get_result::<AccountModel>(&mut conn)?,
        )
    }
    Ok(results)
}
