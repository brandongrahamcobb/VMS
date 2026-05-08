use crate::db::schema::cash_equipment_set;
use crate::models::wz::equip::model::Equip;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = cash_equipment_set)]
pub struct NewCharacterCashEquipmentSetInsert {
    pub char_id: i32,
}

#[derive(Clone, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = cash_equipment_set)]
pub struct CashEquipmentSetModel {
    pub char_id: i32,
    pub hat_id: i32,
    pub face_acc_id: i32,
    pub eye_acc_id: i32,
    pub ear_acc_id: i32,
    pub top_id: i32,
    pub bottom_id: i32,
    pub shoes_id: i32,
    pub gloves_id: i32,
    pub cape_id: i32,
    pub weapon_id: i32,
    pub ring_one_id: i32,
    pub ring_two_id: i32,
    pub ring_three_id: i32,
    pub ring_four_id: i32,
    pub pendant_id: i32,
    pub belt_id: i32,
    pub shoulder_id: i32,
    pub subweapon_id: i32,
    pub hair_id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct CashEquipmentSet {
    pub model: CashEquipmentSetModel,
    pub hat: Equip,
    pub face_acc: Equip,
    pub eye_acc: Equip,
    pub ear_acc: Equip,
    pub top: Equip,
    pub bottom: Equip,
    pub shoes: Equip,
    pub gloves: Equip,
    pub cape: Equip,
    pub weapon: Equip,
    pub ring_one: Equip,
    pub ring_two: Equip,
    pub ring_three: Equip,
    pub ring_four: Equip,
    pub pendant: Equip,
    pub belt: Equip,
    pub shoulder: Equip,
    pub subweapon: Equip,
    pub hair: Equip,
}
