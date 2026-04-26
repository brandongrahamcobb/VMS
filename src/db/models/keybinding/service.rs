use crate::db::models::keybinding::core::{Keybinding, NewKeybinding};
use crate::db::schema::keybindings;
use crate::db::schema::keybindings::dsl::*;
use crate::runtime::relay::RuntimeContext;
use diesel::expression_methods::*;
use diesel::pg::upsert::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub fn get_keybindings_by_characterid(
    ctx: &RuntimeContext,
    c_id: i32,
) -> QueryResult<Vec<Keybinding>> {
    let mut conn = ctx.shared_state.db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    keybindings::table
        .filter(keybindings::character_id.eq(c_id))
        .load::<Keybinding>(&mut conn)
}

pub fn update_keybindings(
    ctx: &RuntimeContext,
    bindings: Vec<NewKeybinding>,
) -> QueryResult<Vec<Keybinding>> {
    let mut conn = ctx.shared_state.db.get().map_err(|e| {
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
