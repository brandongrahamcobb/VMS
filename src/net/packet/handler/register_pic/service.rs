use crate::models::account;
use crate::net::error::NetworkError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub async fn set_pic(
    state: &SharedState,
    session: Session,
    pic: String,
) -> Result<(), NetworkError> {
    let mut acc = session.acc.clone();
    acc.model.pic = pic;
    account::query::update_by_model(state, acc.model).await?;
    Ok(())
}
