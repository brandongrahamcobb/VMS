use crate::models::account::model::Account;
use crate::models::character::model::Character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::chat_text::reader::ChatTextReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChatTextStore {
    pub acc: Account,
    pub char: Character,
    pub is_empty: bool,
    pub msg: String,
    pub show: i16,
}

impl ChatTextStore {
    pub async fn store_chat_text(
        state: &SharedState,
        session: Session,
        reader: ChatTextReader,
    ) -> Result<Self, NetworkError> {
        let acc = session.get_acc()?;
        let char = session.get_char()?;
        return Ok(Self {
            acc,
            char,
            is_empty: reader.is_empty,
            msg: reader.msg.clone(),
            show: reader.show,
        });
    }
}
