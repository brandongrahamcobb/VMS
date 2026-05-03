use crate::db::schema::{
    android_equipment_set, cash_equipment_set, pet_equipment_set, regular_equipment_set,
};
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = regular_equipment_set)]
pub struct NewRegularEquipmentSet {
    pub char_id: i32,
    pub top: i32,
    pub bottom: i32,
    pub shoes: i32,
    pub weapon: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = cash_equipment_set)]
pub struct NewCashEquipmentSet {
    pub char_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = android_equipment_set)]
pub struct NewAndroidEquipmentSet {
    pub char_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = pet_equipment_set)]
pub struct NewPetEquipmentSet {
    pub char_id: i32,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = regular_equipment_set)]
pub struct RegularEquipmentSet {
    pub char_id: i32,
    pub hat: Option<i32>,
    pub face_acc: Option<i32>,
    pub eye_acc: Option<i32>,
    pub ear_acc: Option<i32>,
    pub top: Option<i32>,
    pub bottom: Option<i32>,
    pub shoes: Option<i32>,
    pub gloves: Option<i32>,
    pub cape: Option<i32>,
    pub shield: Option<i32>,
    pub weapon: Option<i32>,
    pub ring_one: Option<i32>,
    pub ring_two: Option<i32>,
    pub ring_three: Option<i32>,
    pub ring_four: Option<i32>,
    pub pendant_one: Option<i32>,
    pub tamed_mob: Option<i32>,
    pub saddle: Option<i32>,
    pub medal: Option<i32>,
    pub belt: Option<i32>,
    pub pocket: Option<i32>,
    pub book: Option<i32>,
    pub pendant_two: Option<i32>,
    pub shoulder: Option<i32>,
    pub android: Option<i32>,
    pub emblem: Option<i32>,
    pub badge: Option<i32>,
    pub subweapon: Option<i32>,
    pub heart: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = cash_equipment_set)]
pub struct CashEquipmentSet {
    pub char_id: i32,
    pub hat: Option<i32>,
    pub face_acc: Option<i32>,
    pub eye_acc: Option<i32>,
    pub ear_acc: Option<i32>,
    pub top: Option<i32>,
    pub bottom: Option<i32>,
    pub shoes: Option<i32>,
    pub gloves: Option<i32>,
    pub cape: Option<i32>,
    pub weapon: Option<i32>,
    pub ring_one: Option<i32>,
    pub ring_two: Option<i32>,
    pub ring_three: Option<i32>,
    pub ring_four: Option<i32>,
    pub pendant: Option<i32>,
    pub belt: Option<i32>,
    pub shoulder: Option<i32>,
    pub subweapon: Option<i32>,
    pub hair: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = pet_equipment_set)]
pub struct PetEquipmentSet {
    pub char_id: i32,
    pub pet_one_acc: Option<i32>,
    pub pet_two_acc: Option<i32>,
    pub pet_three_acc: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = android_equipment_set)]
pub struct AndroidEquipmentSet {
    pub char_id: i32,
    pub android_hat: Option<i32>,
    pub android_face: Option<i32>,
    pub android_top: Option<i32>,
    pub android_bottom: Option<i32>,
    pub android_gloves: Option<i32>,
    pub android_cape: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}
