use crate::models::account;
use crate::models::account::model::Account;
use crate::net::error::NetworkError;
use crate::net::packet::handler::tos::reader::TosReader;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct TosStore {
    acc: Account,
    accepted: bool,
}

impl TosStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_tos(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &TosReader,
    ) -> Result<Self, NetworkError> {
        let accepted: bool = reader.confirmed != 0x01;
        let mut acc: Account = session.acc.ok_or(SessionError::NoAccount(session.id))?;
        acc.accepted_tos = true;
        account::query::update(&state, &acc).await?;
        Ok(Self {
            acc: acc.clone(),
            accepted: accepted.clone(),
        })
    }
}
