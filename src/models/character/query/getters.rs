use crate::db::schema::{character_limits, characters};
use crate::models::character::model::CharacterModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_character_models_by_account_id(
    state: &SharedState,
    acc_id: i32,
) -> QueryResult<Vec<CharacterModel>> {
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
    characters::table
        .filter(characters::acc_id.eq(acc_id))
        .load::<CharacterModel>(&mut conn)
}

pub async fn get_character_model_by_name(
    state: &SharedState,
    ign: String,
) -> QueryResult<CharacterModel> {
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
    characters::table
        .filter(characters::ign.eq(ign))
        .first::<CharacterModel>(&mut conn)
}

pub async fn get_character_model_by_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<CharacterModel> {
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
    characters::table
        .filter(characters::id.eq(char_id))
        .first::<CharacterModel>(&mut conn)
}

pub async fn get_character_max_by_account_and_world_id(
    state: &SharedState,
    acc_id: i32,
    world_id: i16,
) -> QueryResult<i16> {
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
    character_limits::table
        .filter(&character_limits::acc_id.eq(acc_id))
        .filter(&character_limits::world_id.eq(world_id as i16))
        .select(&character_limits::char_max)
        .first(&mut conn)
}
