use crate::db::schema::skills;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = skills)]
pub struct Skill {
    pub id: i32,
    pub wz_id: i32,
    pub char_id: i32,
    pub level: u8,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}
