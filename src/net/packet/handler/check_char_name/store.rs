use crate::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::check_char_name::reader::CheckCharNameReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct CheckCharNameStore {
    pub exists: bool,
    pub ign: String,
}

impl CheckCharNameStore {
    pub async fn store_check_char_name(
        state: &SharedState,
        session: Session,
        reader: CheckCharNameReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(session);
        let exists = character::query::getters::get_character_model_by_name(state, reader.ign.clone())
            .await
            .is_ok();
        Ok(Self {
            exists,
            ign: reader.ign.clone(),
        })
    }
}
