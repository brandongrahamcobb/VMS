use crate::db::schema::character_limits;
use crate::models::channel::model::Channel;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone)]
pub struct WorldModel {
    pub id: i16,
    pub name: String,
    pub flag: i16,
    pub event_message: String,
}

#[derive(Queryable, AsChangeset)]
#[diesel(table_name = character_limits)]
pub struct CharacterLimit {
    pub acc_id: i32,
    pub char_max: i16,
    pub world_id: i16,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

pub struct WorldInfo {
    pub id: i16,
    pub name: &'static str,
    pub port: i16,
}

#[derive(Clone)]
pub struct World {
    pub model: WorldModel,
    pub channels: Vec<Channel>,
}
