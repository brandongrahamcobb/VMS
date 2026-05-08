use crate::db::schema::character_limits;
use crate::models::channel::model::ChannelModel;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone)]
pub struct WorldModel {
    pub id: i8,
    pub name: String,
    pub flag: i8,
    pub event_message: String,
}

#[derive(Queryable, AsChangeset)]
#[diesel(table_name = character_limits)]
pub struct CharacterLimit {
    pub acc_id: i32,
    pub char_max: i16,
    pub world_id: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

pub struct WorldInfo {
    pub id: i16,
    pub name: &'static str,
    pub port: u16,
}

#[derive(Clone)]
pub struct World {
    pub model: WorldModel,
    pub channels: Vec<Channel>,
}
