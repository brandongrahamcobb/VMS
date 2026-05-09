use crate::db::schema::skills;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = skills)]
pub struct SkillModel {
    pub char_id: i32,
    pub wz_id: i32,
    pub level: i16,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct Skill {
    pub model: SkillModel,
}
