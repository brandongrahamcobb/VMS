use crate::db::schema::android_equipment_sets;
use crate::models::character::equipment_set::android::model::AndroidEquipmentSetModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

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
    android_equipment_sets::table
        .filter(android_equipment_sets::char_id.eq(&char_id))
        .first::<AndroidEquipmentSetModel>(&mut conn)
}
