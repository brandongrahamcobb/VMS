use crate::db::schema::regular_equipment_set;
use crate::models::character::equipment_set::regular::model::{
    NewCharacterRegularEquipmentSetInsert, RegularEquipmentSetModel,
};
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn create_regular_equipment_set_model_for_new_character(
    state: &SharedState,
    regular_equips: NewCharacterRegularEquipmentSetInsert,
) -> QueryResult<RegularEquipmentSetModel> {
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
    diesel::insert_into(regular_equipment_set::table)
        .values(regular_equips)
        .get_result::<RegularEquipmentSetModel>(&mut conn)
}

pub async fn get_regular_equipment_set_model_by_character_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<RegularEquipmentSetModel> {
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
    regular_equipment_set::table
        .filter(regular_equipment_set::char_id.eq(&char_id))
        .first::<RegularEquipmentSetModel>(&mut conn)
}
