use crate::db::schema::equips;
use crate::models::item::equip::model::EquipModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_equip_model_by_id(state: &SharedState, id: i32) -> QueryResult<EquipModel> {
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
        .get_result::<EquipModel>(&mut conn)
}
