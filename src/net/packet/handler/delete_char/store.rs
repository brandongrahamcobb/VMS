use crate::models::{account, character};
use crate::net::packet::handler::credentials::service::StatusCode;
use crate::net::packet::handler::delete_char::reader::DeleteCharReader;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct DeleteCharStore {
    pub acc: Account,
    pub char: Character,
    pub status: bool,
}

impl DeleteCharStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_delete_char(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &DeleteCharReader,
    ) -> Result<Self, NetworkError> {
        let acc = account::query::get_account_by_id(state, &session.acc_id).await?;
        let char = character::query::get_character_by_id(state, &reader.char_id).await?;
        let status = delete_char::service::check_pic(&acc, &reader.pic)?;
        if status {
            character::query::delete_character(state, &acc.id, &char).await?;
        }
        Ok(Self {
            acc: acc.clone(),
            char: char.clone(),
            status: status.clone(),
        })
    }
}
