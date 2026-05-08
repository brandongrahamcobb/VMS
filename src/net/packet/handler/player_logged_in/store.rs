use crate::models::channel::model::ChannelModel;
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
    pub channel_model: ChannelModel,
    pub char: Character,
}

impl PlayerLoggedInStore {
    pub async fn store_player_logged_in(
        state: &SharedState,
        session: Session,
        reader: PlayerLoggedInReader,
    ) -> Result<Self, NetworkError> {
        let char: Character = session.char.clone();
        let channel_model: ChannelModel = session.channel.model.clone();
        let bind_models: Vec<KeybindingModel> =
            keybinding::query::get_keybinding_models_by_character_id(state, reader.char_id).await?;
        Ok(Self {
            bind_models,
            channel_model,
            char,
        })
    }
}
