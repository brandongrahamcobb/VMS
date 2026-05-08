use crate::models::account::model::AccountModel;
use crate::models::character;
use crate::models::character::model::CharacterModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::delete_char;
use crate::net::packet::handler::delete_char::reader::DeleteCharReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct DeleteCharStore {
    pub char_model: CharacterModel,
    pub status: bool,
}

impl DeleteCharStore {
    pub async fn store_delete_char(
        state: &SharedState,
        session: Session,
        reader: DeleteCharReader,
    ) -> Result<Self, NetworkError> {
        let acc_model: AccountModel = session.acc.model.clone();
        let char_model = character::query::get_character_model_by_id(state, reader.char_id).await?;
        let status = delete_char::service::check_pic(acc_model.clone(), reader.pic)?;
        if status {
            character::query::delete_character_by_id(state, char_model.id).await?;
        }
        Ok(Self { char_model, status })
    }
}
