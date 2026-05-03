use crate::db::schema::characters;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Identifiable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = characters)]
pub struct Character {
    pub id: i32,
    pub acc_id: i32,
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

#[derive(Insertable)]
#[diesel(table_name = characters)]
pub struct NewCharacter {
    pub acc_id: i32,
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
