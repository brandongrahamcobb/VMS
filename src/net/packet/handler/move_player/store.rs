use crate::models::character::model::Character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::move_player::reader::MovePlayerReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct MovePlayerStore {
    pub char: Character,
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
        let char: Character = session.get_char()?;
        Ok(Self {
            char: char.clone(),
            movement_bytes: reader.movement_bytes.clone(),
            too_short: reader.too_short,
            empty: reader.empty,
        })
    }
}
