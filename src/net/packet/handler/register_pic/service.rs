use crate::db::error::DatabaseError;
use crate::models::account;
use crate::net::error::NetworkError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub async fn set_pic(
    state: SharedState,
    session: Session,
    pic: String,
) -> Result<(), NetworkError> {
    let acc_id = session.acc_id;
    let mut acc = account::query::get_account_by_id(state.clone(), &acc_id)
        .await
        .map_err(DatabaseError::from)
        .map_err(NetworkError::from)?;
    acc.pic = Some(pic);
    account::query::update(state.clone(), &acc)
        .await
        .map_err(DatabaseError::from)
        .map_err(NetworkError::from)?;
    Ok(())
}
