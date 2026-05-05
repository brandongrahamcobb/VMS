use crate::models::account;
use crate::net::error::NetworkError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub async fn set_pic(
    state: &SharedState,
    session: &Session,
    pic: &str,
) -> Result<(), NetworkError> {
    let acc_id = session.acc_id;
    let mut acc = account::query::get_account_by_id(state, &acc_id).await?;
    acc.pic = Some(pic.to_string());
    account::query::update(state, &acc).await?;
    Ok(())
}
