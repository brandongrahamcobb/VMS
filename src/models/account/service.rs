use std::time::SystemTime;

use crate::models::account;
use crate::models::account::error::AccountError;
use crate::models::account::model::Account;
use crate::models::account::model::AccountModel;
use crate::models::character;
use crate::models::character::model::Character;
use crate::models::error::ModelError;
use crate::runtime::state::SharedState;

impl Account {
    pub async fn accept_tos(&self, state: &SharedState) -> Result<Self, ModelError> {
        account::query::setters::accept_tos_by_account_id(state, self.model.get_id()?).await?;
        Ok(self.clone())
    }

    pub async fn set_pic(&self, state: &SharedState, pic: String) -> Result<Self, ModelError> {
        account::query::setters::set_pic_by_account_id(state, self.model.get_id()?, pic.clone())
            .await?;
        Ok(self.clone())
    }
}

pub async fn get_account_by_id(state: &SharedState, acc_id: i32) -> Result<Account, ModelError> {
    let acc_model = account::query::getters::get_account_model_by_id(state, acc_id).await?;
    let acc = acc_model.load(state).await?;
    Ok(acc)
}

impl AccountModel {
    pub async fn load(&self, state: &SharedState) -> Result<Account, ModelError> {
        let acc_id = self.get_id()?;
        let char_models =
            character::query::getters::get_character_models_by_account_id(state, acc_id).await?;
        let mut chars: Vec<Character> = Vec::<Character>::new();
        for char_model in char_models {
            chars.push(char_model.load(state).await?);
        }
        let acc = Account {
            model: self.clone(),
            chars: chars.clone(),
        };
        Ok(acc)
    }

    pub fn get_pic(&self) -> Result<String, ModelError> {
        if let Some(pic) = self.pic.clone() {
            Ok(pic)
        } else {
            Err(ModelError::from(AccountError::NoPic(self.get_id()?)))
        }
    }

    pub fn get_id(&self) -> Result<i32, ModelError> {
        if let Some(id) = self.id {
            Ok(id)
        } else {
            Err(ModelError::from(AccountError::NoId))
        }
    }

    pub fn get_created_at(&self) -> Result<SystemTime, ModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(ModelError::from(AccountError::NoCreatedAt(self.get_id()?)))
        }
    }
}

pub async fn get_account_by_username(
    state: &SharedState,
    username: String,
) -> Result<Account, ModelError> {
    let acc_model =
        account::query::getters::get_account_model_by_username(state, username.clone()).await?;
    let acc = acc_model.load(state).await?;
    Ok(acc)
}
