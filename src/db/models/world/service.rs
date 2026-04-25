use crate::db::schema::character_limits;
use crate::runtime::relay::RuntimeContext;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub fn get_character_max_by_account_and_world_id(
    ctx: &RuntimeContext,
    account_id: i64,
    world_id: i16,
) -> QueryResult<i32> {
    let mut conn = ctx.shared_state.db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    character_limits::table
        .filter(character_limits::account_id.eq(account_id))
        .filter(character_limits::world_id.eq(world_id))
        .select(character_limits::char_max)
        .first(&mut conn)
}
