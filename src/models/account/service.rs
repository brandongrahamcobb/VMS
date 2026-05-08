use crate::models::account::model::AccountModel;
use std::time::SystemTime;

impl AccountModel {
    pub fn new() -> Self {
        Self {
            id: -1,
            username: String::new(),
            password: String::new(),
            pin: String::new(),
            pic: String::new(),
            last_login_at: SystemTime::now(),
            gender_id: -1,
            accepted_tos: false,
            banned: false,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}
