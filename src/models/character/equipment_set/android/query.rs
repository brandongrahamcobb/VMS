use crate::db::schema::android_equipment_set;
use crate::models::character::equipment_set::android::model::{
    AndroidEquipmentSetModel, NewCharacterAndroidEquipmentSetInsert,
};
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn create_android_equipment_set_model_for_new_character(
    state: &SharedState,
    android_equips: NewCharacterAndroidEquipmentSetInsert,
) -> QueryResult<AndroidEquipmentSetModel> {
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
    diesel::insert_into(android_equipment_set::table)
        .values(android_equips)
        .get_result::<AndroidEquipmentSetModel>(&mut conn)
}

pub async fn get_android_equipment_set_model_by_character_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<AndroidEquipmentSetModel> {
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
    android_equipment_set::table
        .filter(android_equipment_set::char_id.eq(&char_id))
        .first::<AndroidEquipmentSetModel>(&mut conn)
}
