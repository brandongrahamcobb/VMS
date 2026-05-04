use crate::db::schema::accounts;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Identifiable, Queryable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub pin: Option<String>,
    pub pic: Option<String>,
    pub last_login_at: Option<SystemTime>,
    pub gender_id: i16,
    pub accepted_tos: bool,
    pub banned: bool,
    pub session_id: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}
