use crate::db::schema::pet_equipment_sets;
use crate::models::character::equipment_set::pet::model::PetEquipmentSetModel;
use crate::runtime::state::SharedState;
use diesel::{QueryResult, RunQueryDsl};

pub async fn update_pet_equips(
    state: &SharedState,
    pet_equip_set_models: Vec<PetEquipmentSetModel>,
) -> QueryResult<Vec<PetEquipmentSetModel>> {
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
    for pet_equip_set_model in &pet_equip_set_models {
        results.push(
            diesel::insert_into(pet_equipment_sets::table)
                .values(pet_equip_set_model)
                .on_conflict(pet_equipment_sets::char_id)
                .do_update()
                .set(pet_equip_set_model)
                .get_result::<PetEquipmentSetModel>(&mut conn)?,
        )
    }
    Ok(results)
}
