use crate::db::schema::android_equipment_sets;
use crate::models::character::equipment_set::android::model::AndroidEquipmentSetModel;
use crate::runtime::state::SharedState;
use diesel::{QueryResult, RunQueryDsl};

pub async fn update_android_equips(
    state: &SharedState,
    android_equip_set_models: Vec<AndroidEquipmentSetModel>,
) -> QueryResult<Vec<AndroidEquipmentSetModel>> {
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
    let mut results = Vec::new();
    for android_equip_set_model in &android_equip_set_models {
        results.push(
            diesel::insert_into(android_equipment_sets::table)
                .values(android_equip_set_model)
                .on_conflict(android_equipment_sets::char_id)
                .do_update()
                .set(android_equip_set_model)
                .get_result::<AndroidEquipmentSetModel>(&mut conn)?,
        )
    }
    Ok(results)
}
