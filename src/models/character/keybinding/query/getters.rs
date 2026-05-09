use crate::db::schema::keybindings;
use crate::models::character::keybinding::model::KeybindingModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
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
