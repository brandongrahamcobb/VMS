use crate::models::account;
use crate::models::account::model::{Account, AccountModel};
use crate::net::error::NetworkError;
use crate::net::packet::handler::tos::reader::TosReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct TosStore {
    pub acc: Account,
    pub accepted: bool,
}

impl TosStore {
    pub async fn store_tos(
        state: &SharedState,
        session: Session,
        reader: TosReader,
    ) -> Result<Self, NetworkError> {
        let accepted: bool = reader.confirmed != 0x01;
        let acc: Account = session.get_acc()?;
        let mut acc_model: AccountModel = acc.model.clone();
        acc_model.accepted_tos = true;
        account::query::update_by_model(state, acc_model.clone()).await?;
        Ok(Self { acc, accepted })
    }
}
