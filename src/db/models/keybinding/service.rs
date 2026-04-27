use crate::db::models::keybinding::core::{Keybinding, NewKeybinding};
use crate::db::schema::keybindings;
use crate::db::schema::keybindings::dsl::*;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::pg::upsert::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_keybindings_by_character_id(
    state: SharedState,
    c_id: i32,
) -> QueryResult<Vec<Keybinding>> {
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
        .filter(keybindings::character_id.eq(c_id))
        .load::<Keybinding>(&mut conn)
}

pub async fn update_keybindings(
    state: SharedState,
    bindings: Vec<NewKeybinding>,
) -> QueryResult<Vec<Keybinding>> {
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
        .set(key.eq(excluded(key)))
        .get_results(&mut conn)
}
