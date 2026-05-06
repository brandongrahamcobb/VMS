use crate::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::move_player::reader::MovePlayerReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct MovePlayerStore {
    pub char: Character,
    pub movement_bytes: Vec<u8>,
}

impl MovePlayerStore {
    pub fn new() -> Self {
        Self
    }

    pub fn store_move_player(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &MovePlayerReader,
    ) -> Result<Self, NetworkError> {
        let char_id = session
            .char_id
            .ok_or(SessionError::NoCharacterSelected(session.id))?;
        let char = character::query::get_character_by_id(state, &char_id)?;
        let movement_bytes = &reader.movement_fragment.to_vec();
        Ok(Self {
            char: char.clone(),
            movement_bytes: movement_bytes.clone(),
        })
    }
}
