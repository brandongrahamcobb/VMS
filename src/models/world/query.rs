use crate::db::schema::character_limits;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_character_max_by_account_and_world_id(
    state: SharedState,
    acc_id: i32,
    world_id: i16,
) -> QueryResult<i32> {
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
        .filter(character_limits::acc_id.eq(acc_id))
        .filter(character_limits::world_id.eq(world_id))
        .select(character_limits::char_max)
        .first(&mut conn)
}
