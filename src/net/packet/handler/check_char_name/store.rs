use crate::models::character;
use crate::net::packet::handler::check_char_name::read::CheckCharNameRead;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

pub struct CheckCharNameStore {
    pub exists: bool,
    pub ign: String,
}

impl CheckCharNameStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_check_char_name(
        &self,
        state: &SharedState,
        session: &Session,
        read: &CheckCharNameRead,
    ) -> Result<Self, NetworkError> {
        let exists = character::query::get_character_by_name(state, &read.ign)
            .await
            .is_ok();
        Ok(Self {
            exists: exists.clone(),
            ign: read.ign.clone(),
        })
    }
}
