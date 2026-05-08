use crate::models::account;
use crate::models::account::error::AccountError;
use crate::models::account::model::Account;
use crate::models::account::model::AccountModel;
use crate::models::account::model::NewAccountInsert;
use crate::models::character;
use crate::models::character::model::Character;
use crate::models::error::ModelError;
use crate::runtime::state::SharedState;

impl NewAccountInsert {
    pub fn default(username: String, password: String, gender_id: i16) -> Self {
        Self {
            username,
            password,
            gender_id,
        }
    }
}

impl Account {
    pub fn new(chars: Vec<Character>, model: AccountModel) -> Self {
        Self { model, chars }
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

impl AccountModel {
    pub fn get_pic(&self) -> Result<String, ModelError> {
        if let Some(pic) = self.pic.clone() {
            Ok(pic)
        } else {
            Err(ModelError::from(AccountError::NoPic(self.id)))
        }
    }
}
