use crate::models::account::model::Account;
use crate::models::character;
use crate::models::character::model::{Character, CharacterModel};
use crate::net::error::NetworkError;
use crate::net::packet::handler::delete_char;
use crate::net::packet::handler::delete_char::reader::DeleteCharReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct DeleteCharStore {
    pub char: Character,
    pub status: bool,
}

impl DeleteCharStore {
    pub async fn store_delete_char(
        state: &SharedState,
        session: Session,
        reader: DeleteCharReader,
    ) -> Result<Self, NetworkError> {
        let acc: Account = session.get_acc()?;
        let char_model: CharacterModel =
            character::query::getters::get_character_model_by_id(state, reader.char_id).await?;
        let char: Character = char_model.load(state).await?;
        let status = delete_char::service::check_pic(acc.model.clone(), reader.pic)?;
        if status {
            character::query::setters::delete_character_by_id(state, char_model.get_id()?).await?;
        }
        Ok(Self { char, status })
    }
}
