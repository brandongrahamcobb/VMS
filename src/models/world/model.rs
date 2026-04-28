use crate::db::schema::{character_limits, worlds};
use crate::models::channel::model::Channel;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Identifiable, Queryable)]
#[diesel(table_name = worlds)]
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
    pub account_id: i64,
    pub char_max: i32,
    pub updated_at: SystemTime,
    pub world_id: i16,
}

pub struct WorldInfo {
    pub id: i16,
    pub name: &'static str,
}
