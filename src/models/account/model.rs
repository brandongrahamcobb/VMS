use crate::db::schema::accounts;
use crate::models::character::model::Character;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Identifiable, Queryable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountModel {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub pin: Option<String>,
    pub pic: Option<String>,
    pub last_login_at: Option<SystemTime>,
    pub gender_id: i16,
    pub accepted_tos: bool,
    pub banned: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct Account {
    pub model: AccountModel,
    pub chars: Vec<Character>,
}

pub struct NewAccountInsert {
    pub username: String,
    pub password: String,
    pub gender_id: i16,
}
