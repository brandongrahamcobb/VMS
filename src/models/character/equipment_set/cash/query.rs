use crate::db::schema::cash_equipment_set;
use crate::models::character::equipment_set::cash::model::{
    CashEquipmentSetModel, NewCharacterCashEquipmentSetInsert,
};
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn create_cash_equipment_set_model_for_new_character(
    state: &SharedState,
    cash_equips: &NewCharacterCashEquipmentSetInsert,
) -> QueryResult<CashEquipmentSetModel> {
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
    diesel::insert_into(cash_equipment_set::table)
        .values(cash_equips)
        .get_result::<CashEquipmentSetModel>(&mut conn)
}

pub async fn get_cash_equipment_set_model_by_character_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<CashEquipmentSetModel> {
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
    cash_equipment_set::table
        .filter(cash_equipment_set::char_id.eq(&char_id))
        .first::<CashEquipmentSetModel>(&mut conn)
}
