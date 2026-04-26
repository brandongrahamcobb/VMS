use crate::db::models::character::core::{Character, NewCharacter};
use crate::db::schema::characters;
use crate::runtime::relay::RuntimeContext;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub fn get_characters_by_accountid(
    ctx: &RuntimeContext,
    account_id: i64,
) -> QueryResult<Vec<Character>> {
    let mut conn = ctx.shared_state.db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    characters::table
        .filter(characters::account.eq(account_id))
        .load::<Character>(&mut conn)
}

pub fn create_character(ctx: &RuntimeContext, char: &NewCharacter) -> QueryResult<Character> {
    let mut conn = ctx.shared_state.db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    diesel::insert_into(characters::table)
        .values(char)
        .get_result::<Character>(&mut conn)
}

pub fn get_character_by_name(ctx: &RuntimeContext, ign: &str) -> QueryResult<Character> {
    let mut conn = ctx.shared_state.db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    characters::table
        .filter(characters::ign.eq(ign))
        .first::<Character>(&mut conn)
}
