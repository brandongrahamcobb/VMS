use crate::db::schema::keybindings;
use crate::models::character::keybinding::model::KeybindingModel;
use crate::runtime::state::SharedState;
use diesel::{QueryResult, RunQueryDsl};

pub async fn update_keybindings(
    state: &SharedState,
    bind_models: Vec<KeybindingModel>,
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
    let mut results = Vec::new();
    for bind_model in &bind_models {
        results.push(
            diesel::insert_into(keybindings::table)
                .values(bind_model)
                .on_conflict(keybindings::char_id)
                .do_update()
                .set(bind_model)
                .get_result::<KeybindingModel>(&mut conn)?,
        )
    }
    Ok(results)
}
