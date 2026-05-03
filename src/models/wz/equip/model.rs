use crate::db::schema::equips;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Identifiable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = equips)]
pub struct Equip {
    pub id: i32,
    pub wz_id: i32,
    pub strength: Option<i32>,
    pub dexterity: Option<i32>,
    pub intelligence: Option<i32>,
    pub luck: Option<i32>,
    pub attack: Option<i32>,
    pub weapon_defense: Option<i32>,
    pub magic: Option<i32>,
    pub magic_defense: Option<i32>,
    pub hp: Option<i32>,
    pub mp: Option<i32>,
    pub accuracy: Option<i32>,
    pub avoid: Option<i32>,
    pub hands: Option<i32>,
    pub speed: Option<i32>,
    pub jump: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Insertable, AsChangeset, Selectable)]
#[diesel(table_name = equips)]
pub struct NewEquip {
    pub wz_id: i32,
    pub strength: Option<i32>,
    pub dexterity: Option<i32>,
    pub intelligence: Option<i32>,
    pub luck: Option<i32>,
    pub attack: Option<i32>,
    pub weapon_defense: Option<i32>,
    pub magic: Option<i32>,
    pub magic_defense: Option<i32>,
    pub hp: Option<i32>,
    pub mp: Option<i32>,
    pub accuracy: Option<i32>,
    pub avoid: Option<i32>,
    pub hands: Option<i32>,
    pub speed: Option<i32>,
    pub jump: Option<i32>,
}
