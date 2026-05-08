use crate::models::character::model::CharacterModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::move_player::reader::MovePlayerReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct MovePlayerStore {
    pub char_model: CharacterModel,
    pub movement_bytes: Vec<u8>,
    pub too_short: bool,
    pub empty: bool,
}

impl MovePlayerStore {
    pub fn store_move_player(
        state: &SharedState,
        session: Session,
        reader: MovePlayerReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(state);
        let char_model: CharacterModel = session.char.model.clone();
        Ok(Self {
            char_model,
            movement_bytes: reader.movement_bytes.clone(),
            too_short: reader.too_short,
            empty: reader.empty,
        })
    }
}
