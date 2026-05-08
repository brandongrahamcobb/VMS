use crate::db::schema::keybindings;
use crate::models::character::keybinding::model::{KeybindingModel, NewCharacterKeybindingInsert};
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::pg::upsert::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_keybinding_models_by_character_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<Vec<KeybindingModel>> {
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
    keybindings::table
        .filter(keybindings::char_id.eq(char_id))
        .load::<KeybindingModel>(&mut conn)
}

pub async fn update_keybindings(
    state: &SharedState,
    bindings: Vec<NewCharacterKeybindingInsert>,
) -> QueryResult<Vec<KeybindingModel>> {
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
    diesel::insert_into(keybindings::table)
        .values(bindings)
        .on_conflict(on_constraint("key_is_unique_per_character"))
        .do_update()
        .set((
            keybindings::bind_type.eq(excluded(keybindings::bind_type)),
            keybindings::action.eq(excluded(keybindings::action)),
        ))
        .get_results(&mut conn)
}
