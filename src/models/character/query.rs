use crate::db::schema::characters;
// use crate::models::character::equipment_set::android::model::AndroidEquipmentSetModel;
// use crate::models::character::equipment_set::cash::model::CashEquipmentSetModel;
// use crate::models::character::equipment_set::pet::model::PetEquipmentSetModel;
// use crate::models::character::equipment_set::regular::model::RegularEquipmentSetModel;
use crate::models::character::model::{CharacterModel, NewCharacterInsert};
use crate::models::error::ModelError;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_characters_models_by_account_id_and_world_id(
    state: &SharedState,
    acc_id: i32,
    world_id: i8,
) -> Result<Vec<CharacterModel>, ModelError> {
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
    characters::table
        .filter(characters::acc_id.eq(acc_id))
        .filter(characters::world_id.eq(world_id as i16))
        .load::<CharacterModel>(&mut conn)?
}

pub async fn create_character_model(
    state: &SharedState,
    insert: &NewCharacterInsert,
) -> QueryResult<CharacterModel> {
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
    diesel::insert_into(characters::table)
        .values(insert)
        .get_result::<CharacterModel>(&mut conn)
}

pub async fn get_character_model_by_name(
    state: &SharedState,
    ign: String,
) -> QueryResult<CharacterModel> {
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
    characters::table
        .filter(characters::ign.eq(ign))
        .first::<CharacterModel>(&mut conn)
}

pub async fn get_character_model_by_id(
    state: &SharedState,
    char_id: i32,
) -> Result<CharacterModel, ModelError> {
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
    characters::table
        .filter(characters::id.eq(char_id))
        .first::<CharacterModel>(&mut conn)?
}

pub async fn delete_character_by_id(state: &SharedState, char_id: i32) -> QueryResult<usize> {
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
    diesel::delete(characters::table.filter(characters::id.eq(char_id))).execute(&mut conn)
}
