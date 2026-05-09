use crate::db::schema::cash_equipment_sets;
use crate::models::character::equipment_set::cash::model::CashEquipmentSetModel;
use crate::runtime::state::SharedState;
use diesel::{QueryResult, RunQueryDsl};

pub async fn update_cash_equips(
    state: &SharedState,
    cash_equip_set_models: Vec<CashEquipmentSetModel>,
) -> QueryResult<Vec<CashEquipmentSetModel>> {
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
    for cash_equip_set_model in &cash_equip_set_models {
        results.push(
            diesel::insert_into(cash_equipment_sets::table)
                .values(cash_equip_set_model)
                .on_conflict(cash_equipment_sets::char_id)
                .do_update()
                .set(cash_equip_set_model)
                .get_result::<CashEquipmentSetModel>(&mut conn)?,
        )
    }
    Ok(results)
}
