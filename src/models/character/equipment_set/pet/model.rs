use crate::db::schema::pet_equipment_sets;
use crate::models::item::equip::model::Equip;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = pet_equipment_sets)]
pub struct PetEquipmentSetModel {
    pub char_id: i32,
    pub accessory_one_id: Option<i32>,
    pub accessory_two_id: Option<i32>,
    pub accessory_three_id: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct PetEquipmentSet {
    pub model: PetEquipmentSetModel,
    pub accessory_one: Option<Equip>,
    pub accessory_two: Option<Equip>,
    pub accessory_three: Option<Equip>,
}
