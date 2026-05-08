use crate::db::schema::pet_equipment_set;
use crate::models::wz::equip::model::Equip;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = pet_equipment_set)]
pub struct NewCharacterPetEquipmentSetInsert {
    pub char_id: i32,
}

#[derive(Clone, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = pet_equipment_set)]
pub struct PetEquipmentSetModel {
    pub char_id: i32,
    pub accessory_one_id: i32,
    pub accessory_two_id: i32,
    pub accessory_three_id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct PetEquipmentSet {
    pub model: PetEquipmentSetModel,
    pub accessory_one: Equip,
    pub accessory_two: Equip,
    pub accessory_three: Equip,
}
