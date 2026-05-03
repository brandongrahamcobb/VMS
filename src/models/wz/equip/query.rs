use crate::db::schema::equips;
use crate::models::wz::equip::model::{Equip, NewEquip};
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn create_equip(state: SharedState, equip: NewEquip) -> QueryResult<Equip> {
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
    diesel::insert_into(equips::table)
        .values(equip)
        .get_result::<Equip>(&mut conn)
}

pub async fn get_equip_by_id(state: SharedState, id: i32) -> QueryResult<Equip> {
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
    equips::table
        .filter(equips::id.eq(id))
        .get_result::<Equip>(&mut conn)
}
