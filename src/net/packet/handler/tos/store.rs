use crate::models::account::model::Account;
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
        let accepted: bool = reader.confirmed == 0x01;
        let acc: Account = session.get_acc()?;
        if accepted {
            acc.accept_tos(state).await?;
        }
        Ok(Self { acc, accepted })
    }
}
