use crate::db::schema::cash_equipment_set;
use crate::models::wz::equip::model::Equip;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = cash_equipment_set)]
pub struct NewCashEquipmentSetInsert {
    pub char_id: i32,
    pub hat_id: Option<i32>,
    pub face_acc_id: Option<i32>,
    pub eye_acc_id: Option<i32>,
    pub ear_acc_id: Option<i32>,
    pub top_id: Option<i32>,
    pub bottom_id: Option<i32>,
    pub shoes_id: Option<i32>,
    pub gloves_id: Option<i32>,
    pub cape_id: Option<i32>,
    pub weapon_id: Option<i32>,
    pub ring_one_id: Option<i32>,
    pub ring_two_id: Option<i32>,
    pub ring_three_id: Option<i32>,
    pub ring_four_id: Option<i32>,
    pub pendant_id: Option<i32>,
    pub belt_id: Option<i32>,
    pub shoulder_id: Option<i32>,
    pub subweapon_id: Option<i32>,
    pub hair_id: Option<i32>,
}

#[derive(Clone, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = cash_equipment_set)]
pub struct CashEquipmentSetModel {
    pub char_id: i32,
    pub hat_id: Option<i32>,
    pub face_acc_id: Option<i32>,
    pub eye_acc_id: Option<i32>,
    pub ear_acc_id: Option<i32>,
    pub top_id: Option<i32>,
    pub bottom_id: Option<i32>,
    pub shoes_id: Option<i32>,
    pub gloves_id: Option<i32>,
    pub cape_id: Option<i32>,
    pub weapon_id: Option<i32>,
    pub ring_one_id: Option<i32>,
    pub ring_two_id: Option<i32>,
    pub ring_three_id: Option<i32>,
    pub ring_four_id: Option<i32>,
    pub pendant_id: Option<i32>,
    pub belt_id: Option<i32>,
    pub shoulder_id: Option<i32>,
    pub subweapon_id: Option<i32>,
    pub hair_id: Option<i32>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct CashEquipmentSet {
    pub model: CashEquipmentSetModel,
    pub hat: Option<Equip>,
    pub face_acc: Option<Equip>,
    pub eye_acc: Option<Equip>,
    pub ear_acc: Option<Equip>,
    pub top: Option<Equip>,
    pub bottom: Option<Equip>,
    pub shoes: Option<Equip>,
    pub gloves: Option<Equip>,
    pub cape: Option<Equip>,
    pub weapon: Option<Equip>,
    pub ring_one: Option<Equip>,
    pub ring_two: Option<Equip>,
    pub ring_three: Option<Equip>,
    pub ring_four: Option<Equip>,
    pub pendant: Option<Equip>,
    pub belt: Option<Equip>,
    pub shoulder: Option<Equip>,
    pub subweapon: Option<Equip>,
    pub hair: Option<Equip>,
}
