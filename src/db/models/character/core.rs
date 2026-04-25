use crate::db::schema::characters;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Identifiable, Queryable, AsChangeset)]
#[diesel(table_name = characters)]
pub struct Character {
    pub id: i32,
    pub account: i64,
    pub world: i16,
    pub ign: String,
    pub level: i16,
    pub exp: i32,
    pub strength: i16,
    pub dexterity: i16,
    pub luck: i16,
    pub intelligence: i16,
    pub hp: i16,
    pub mp: i16,
    pub max_hp: i16,
    pub max_mp: i16,
    pub ap: i16,
    pub fame: i16,
    pub meso: i32,
    pub job: i16,
    pub face: i32,
    pub hair: i32,
    pub hair_color: i32,
    pub skin: i32,
    pub gender: i16,
    pub created_at: SystemTime,
    pub map: i32,
    pub updated_at: SystemTime,
}
