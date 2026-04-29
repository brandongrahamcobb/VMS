use crate::db::schema::character_limits;
use crate::models::channel::model::Channel;
use diesel::prelude::*;
use std::time::SystemTime;

pub struct World {
    pub id: i16,
    pub name: String,
    pub flag: i8,
    pub event_message: String,
    pub channels: Vec<Channel>,
}

#[derive(Queryable, AsChangeset)]
#[diesel(table_name = character_limits)]
pub struct CharacterLimit {
    pub acc_id: i64,
    pub char_max: i32,
    pub world_id: i16,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

pub struct WorldInfo {
    pub id: i16,
    pub name: &'static str,
    pub port: i16,
}
