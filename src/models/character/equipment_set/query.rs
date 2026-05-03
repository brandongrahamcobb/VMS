use crate::db::schema::{android_equipment_set, cash_equipment_set, pet_equipment_set, regular_equipment_set};
use crate::models::character::equipment_set::model::{
    AndroidEquipmentSet, CashEquipmentSet, NewAndroidEquipmentSet, NewCashEquipmentSet, NewPetEquipmentSet, NewRegularEquipmentSet, PetEquipmentSet, RegularEquipmentSet
};
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn create_regular_equipment_set_for_new_character(
    state: SharedState,
    regular_equips: &NewRegularEquipmentSet,
) -> QueryResult<RegularEquipmentSet> {
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
        .get_result::<RegularEquipmentSet>(&mut conn)
}

pub async fn create_cash_equipment_set_for_new_character(
    state: SharedState,
    cash_equips: &NewCashEquipmentSet,
) -> QueryResult<CashEquipmentSet> {
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
        .get_result::<CashEquipmentSet>(&mut conn)
}

pub async fn create_android_equipment_set_for_new_character(
    state: SharedState,
    android_equips: &NewAndroidEquipmentSet,
) -> QueryResult<AndroidEquipmentSet> {
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
        .get_result::<AndroidEquipmentSet>(&mut conn)
}

pub async fn create_pet_equipment_set_for_new_character(
    state: SharedState,
    pet_equips: &NewPetEquipmentSet,
) -> QueryResult<PetEquipmentSet> {
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
    diesel::insert_into(pet_equipment_set::table)
        .values(pet_equips)
        .get_result::<PetEquipmentSet>(&mut conn)
}

pub async fn get_regular_equipment_set_by_character_id(
    state: SharedState,
    char_id: i32,
) -> QueryResult<RegularEquipmentSet> {
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
        .filter(regular_equipment_set::char_id.eq(char_id))
        .first::<RegularEquipmentSet>(&mut conn)
}

pub async fn get_cash_equipment_set_by_character_id(
    state: SharedState,
    char_id: i32,
) -> QueryResult<CashEquipmentSet> {
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
        .filter(cash_equipment_set::char_id.eq(char_id))
        .first::<CashEquipmentSet>(&mut conn)
}
