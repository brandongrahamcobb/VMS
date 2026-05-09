use crate::db::schema::characters;
use crate::models::character::model::CharacterModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn update_characters(
    state: &SharedState,
    char_models: Vec<CharacterModel>,
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
    let mut results = Vec::new();
    for char_model in &char_models {
        results.push(
            diesel::insert_into(characters::table)
                .values(char_model)
                .on_conflict(characters::id)
                .do_update()
                .set(char_model)
                .get_result::<CharacterModel>(&mut conn)?,
        )
    }
    Ok(results)
}

pub async fn delete_character_by_id(state: &SharedState, char_id: i32) -> QueryResult<usize> {
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
    diesel::delete(characters::table.filter(characters::id.eq(char_id))).execute(&mut conn)
}
