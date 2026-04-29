use crate::db::schema::{
    android_equipment, cash_equipment, character_equipment, characters, pet_equipment,
};
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Identifiable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = characters)]
pub struct Character {
    pub id: i32,
    pub acc_id: i64,
    pub world_id: i16,
    pub ign: String,
    pub level: Option<i16>,
    pub exp: Option<i32>,
    pub strength: Option<i16>,
    pub dexterity: Option<i16>,
    pub luck: Option<i16>,
    pub intelligence: Option<i16>,
    pub hp: Option<i16>,
    pub mp: Option<i16>,
    pub max_hp: Option<i16>,
    pub max_mp: Option<i16>,
    pub ap: Option<i16>,
    pub fame: Option<i16>,
    pub meso: Option<i32>,
    pub job: i16,
    pub face: i32,
    pub hair: i32,
    pub hair_color: i32,
    pub skin: i32,
    pub gender: i16,
    pub map: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = character_equipment)]
pub struct CharacterEquipment {
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
    pub sub_weapon: Option<i32>,
    pub belt: Option<i32>,
    pub pendant_one: Option<i32>,
    pub pendant_two: Option<i32>,
    pub ring_one: Option<i32>,
    pub ring_two: Option<i32>,
    pub ring_three: Option<i32>,
    pub ring_four: Option<i32>,
    pub shoulder: Option<i32>,
    pub emblem: Option<i32>,
    pub medal: Option<i32>,
    pub badge: Option<i32>,
    pub android: Option<i32>,
    pub heart: Option<i32>,
    pub book: Option<i32>,
    pub pocket: Option<i32>,
    pub totem_one: Option<i32>,
    pub totem_two: Option<i32>,
    pub totem_three: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = cash_equipment)]
pub struct CashEquipment {
    pub char_id: i32,
    pub cash_ring_one: Option<i32>,
    pub cash_ring_two: Option<i32>,
    pub cash_ring_three: Option<i32>,
    pub cash_ring_four: Option<i32>,
    pub cash_hat: Option<i32>,
    pub cash_face: Option<i32>,
    pub cash_hair: Option<i32>,
    pub cash_pendant: Option<i32>,
    pub cash_weapon: Option<i32>,
    pub cash_belt: Option<i32>,
    pub cash_top: Option<i32>,
    pub cash_bottom: Option<i32>,
    pub cash_shoes: Option<i32>,
    pub cash_ear_acc: Option<i32>,
    pub cash_shoulder: Option<i32>,
    pub cash_sub_weapon: Option<i32>,
    pub cash_cape: Option<i32>,
    pub cash_gloves: Option<i32>,
    pub cash_eye_acc: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = pet_equipment)]
pub struct PetEquipment {
    pub char_id: i32,
    pub pet_one_acc: Option<i32>,
    pub pet_two_acc: Option<i32>,
    pub pet_three_acc: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Queryable, AsChangeset, Selectable)]
#[diesel(table_name = android_equipment)]
pub struct AndroidEquipment {
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

#[derive(Insertable)]
#[diesel(table_name = characters)]
pub struct NewCharacter {
    pub acc_id: i64,
    pub world_id: i16,
    pub ign: String,
    pub job: i16,
    pub face: i32,
    pub hair: i32,
    pub hair_color: i32,
    pub skin: i32,
    pub gender: i16,
    pub map: i32,
}

#[derive(Insertable)]
#[diesel(table_name = character_equipment)]
pub struct NewCharacterEquipment {
    pub char_id: i32,
    pub hat: Option<i32>,
    pub top: i32,
    pub bottom: Option<i32>,
    pub shoes: i32,
    pub weapon: i32,
    pub sub_weapon: Option<i32>,
}
