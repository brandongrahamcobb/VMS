use crate::models::account;
use crate::models::account::model::Account;
use crate::net::error::NetworkError;
use crate::runtime::state::SharedState;

pub async fn set_pic(state: &SharedState, acc: Account, pic: String) -> Result<(), NetworkError> {
    let mut acc_model = acc.model.clone();
    acc_model.pic = Some(pic);
    account::query::update_by_model(state, acc_model).await?;
    Ok(())
}
