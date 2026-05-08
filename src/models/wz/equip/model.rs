use crate::db::schema::equips;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Identifiable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = equips)]
pub struct EquipModel {
    pub id: i32,
    pub wz_id: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub luck: i32,
    pub attack: i32,
    pub weapon_defense: i32,
    pub magic: i32,
    pub magic_defense: i32,
    pub hp: i32,
    pub mp: i32,
    pub accuracy: i32,
    pub avoid: i32,
    pub hands: i32,
    pub speed: i32,
    pub jump: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, AsChangeset, Selectable)]
#[diesel(table_name = equips)]
pub struct NewEquipInsert {
    pub wz_id: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub luck: i32,
    pub attack: i32,
    pub weapon_defense: i32,
    pub magic: i32,
    pub magic_defense: i32,
    pub hp: i32,
    pub mp: i32,
    pub accuracy: i32,
    pub avoid: i32,
    pub hands: i32,
    pub speed: i32,
    pub jump: i32,
}

#[derive(Clone)]
pub struct Equip {
    pub model: EquipModel,
}
