use crate::db::schema::regular_equipment_sets;
use crate::models::character::equipment_set::regular::model::RegularEquipmentSetModel;
use crate::runtime::state::SharedState;
use diesel::{QueryResult, RunQueryDsl};

pub async fn update_regular_equips(
    state: &SharedState,
    regular_equip_set_models: Vec<RegularEquipmentSetModel>,
) -> QueryResult<Vec<RegularEquipmentSetModel>> {
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
    for regular_equip_set_model in &regular_equip_set_models {
        results.push(
            diesel::insert_into(regular_equipment_sets::table)
                .values(regular_equip_set_model)
                .on_conflict(regular_equipment_sets::char_id)
                .do_update()
                .set(regular_equip_set_model)
                .get_result::<RegularEquipmentSetModel>(&mut conn)?,
        )
    }
    Ok(results)
}
