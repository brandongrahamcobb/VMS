use crate::{db::schema::android_equipment_set, models::wz::equip::model::Equip};
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = android_equipment_set)]
pub struct NewCharacterAndroidEquipmentSetInsert {
    pub char_id: i32,
}

#[derive(Clone, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = android_equipment_set)]
pub struct AndroidEquipmentSetModel {
    pub char_id: i32,
    pub hat_id: i32,
    pub face_id: i32,
    pub top_id: i32,
    pub bottom_id: i32,
    pub gloves_id: i32,
    pub cape_id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct AndroidEquipmentSet {
    pub model: AndroidEquipmentSetModel,
    pub hat: Equip,
    pub face: Equip,
    pub top: Equip,
    pub bottom: Equip,
    pub gloves: Equip,
    pub cape: Equip,
}
