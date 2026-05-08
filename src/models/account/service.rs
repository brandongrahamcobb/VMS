use crate::models::account;
use crate::models::account::model::Account;
use crate::models::account::model::AccountModel;
use crate::models::character;
use crate::models::error::ModelError;
use crate::runtime::state::SharedState;
use std::time::SystemTime;

impl Account {
    pub fn new() -> Self {
        Self {
            model: AccountModel::new(),
            chars: Vec::new(),
        }
    }
}

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

pub async fn get_account_by_id(state: &SharedState, acc_id: i32) -> Result<Account, ModelError> {
    let acc_model = account::query::get_account_model_by_id(state, acc_id).await?;
    let chars = character::service::get_characters_by_account_id(state, acc_id).await?;
    Ok(Account {
        model: acc_model.clone(),
        chars: chars.clone(),
    })
}
