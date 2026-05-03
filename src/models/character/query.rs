use crate::db::schema::characters;
use crate::models::character::model::{Character, NewCharacter};
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_characters_by_account_id_and_world_id(
    state: SharedState,
    acc_id: i32,
    world_id: i16,
) -> QueryResult<Vec<Character>> {
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
        .filter(characters::world_id.eq(world_id))
        .load::<Character>(&mut conn)
}

pub async fn create_character(state: SharedState, char: &NewCharacter) -> QueryResult<Character> {
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
    diesel::insert_into(characters::table)
        .values(char)
        .get_result::<Character>(&mut conn)
}

pub async fn get_character_by_name(state: SharedState, ign: &str) -> QueryResult<Character> {
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
        .first::<Character>(&mut conn)
}

pub async fn get_character_by_id(state: SharedState, char_id: i32) -> QueryResult<Character> {
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
        .first::<Character>(&mut conn)
}

pub async fn get_account_id_by_character_id(state: SharedState, char_id: i32) -> QueryResult<i32> {
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
        .select(characters::acc_id)
        .first::<i32>(&mut conn)
}

pub async fn delete_character(state: SharedState, acc_id: i32, char_id: i32) -> QueryResult<usize> {
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
    diesel::delete(
        characters::table
            .filter(characters::id.eq(char_id))
            .filter(characters::acc_id.eq(acc_id)),
    )
    .execute(&mut conn)
}
