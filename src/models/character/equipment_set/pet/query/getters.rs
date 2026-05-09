use crate::db::schema::pet_equipment_sets;
use crate::models::character::equipment_set::pet::model::PetEquipmentSetModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_pet_equipment_set_model_by_character_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<PetEquipmentSetModel> {
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
    pet_equipment_sets::table
        .filter(pet_equipment_sets::char_id.eq(&char_id))
        .first::<PetEquipmentSetModel>(&mut conn)
}
