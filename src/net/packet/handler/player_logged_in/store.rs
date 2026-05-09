use crate::models::character::keybinding;
use crate::models::character::keybinding::model::{Keybinding, KeybindingModel};
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct PlayerLoggedInStore {
    pub binds: Vec<Keybinding>,
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
            keybinding::query::getters::get_keybinding_models_by_character_id(
                state,
                reader.char_id,
            )
            .await?;
        let mut binds: Vec<Keybinding> = Vec::<Keybinding>::new();
        for bind_model in bind_models {
            binds.push(bind_model.load()?);
        }
        Ok(Self {
            binds,
            channel,
            char,
        })
    }
}
