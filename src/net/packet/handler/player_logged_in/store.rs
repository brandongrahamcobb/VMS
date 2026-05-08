use crate::models::channel::model::Channel;
use crate::models::character::keybinding;
use crate::models::character::keybinding::model::KeybindingModel;
use crate::models::character::model::Character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct PlayerLoggedInStore {
    pub bind_models: Vec<KeybindingModel>,
    pub channel: Channel,
    pub char: Character,
}

impl PlayerLoggedInStore {
    pub async fn store_player_logged_in(
        state: &SharedState,
        session: Session,
        reader: PlayerLoggedInReader,
    ) -> Result<Self, NetworkError> {
        let char: Character = session.get_char()?;
        let channel: Channel = session.get_channel()?;
        let bind_models: Vec<KeybindingModel> =
            keybinding::query::get_keybinding_models_by_character_id(state, reader.char_id).await?;
        Ok(Self {
            bind_models,
            channel,
            char,
        })
    }
}
