use crate::db::schema::regular_equipment_set;
use crate::models::wz::equip::model::Equip;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = regular_equipment_set)]
pub struct NewCharacterRegularEquipmentSetInsert {
    pub char_id: i32,
    pub top_id: i32,
    pub bottom_id: i32,
    pub shoes_id: i32,
    pub weapon_id: i32,
}

#[derive(Clone, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = regular_equipment_set)]
pub struct RegularEquipmentSetModel {
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
    pub shield_id: i32,
    pub weapon_id: i32,
    pub ring_one_id: i32,
    pub ring_two_id: i32,
    pub ring_three_id: i32,
    pub ring_four_id: i32,
    pub pendant_one_id: i32,
    pub tamed_mob_id: i32,
    pub saddle_id: i32,
    pub medal_id: i32,
    pub belt_id: i32,
    pub pocket_id: i32,
    pub book_id: i32,
    pub pendant_two_id: i32,
    pub shoulder_id: i32,
    pub android_id: i32,
    pub emblem_id: i32,
    pub badge_id: i32,
    pub subweapon_id: i32,
    pub heart_id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct RegularEquipmentSet {
    pub model: RegularEquipmentSetModel,
    pub hat: Equip,
    pub face_acc: Equip,
    pub eye_acc: Equip,
    pub ear_acc: Equip,
    pub top: Equip,
    pub bottom: Equip,
    pub shoes: Equip,
    pub gloves: Equip,
    pub cape: Equip,
    pub shield: Equip,
    pub weapon: Equip,
    pub ring_one: Equip,
    pub ring_two: Equip,
    pub ring_three: Equip,
    pub ring_four: Equip,
    pub pendant_one: Equip,
    pub tamed_mob: Equip,
    pub saddle: Equip,
    pub medal: Equip,
    pub belt: Equip,
    pub pocket: Equip,
    pub book: Equip,
    pub pendant_two: Equip,
    pub shoulder: Equip,
    pub android: Equip,
    pub emblem: Equip,
    pub badge: Equip,
    pub subweapon: Equip,
    pub heart: Equip,
}
