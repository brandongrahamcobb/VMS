use crate::models::character::keybinding::model::NewKeybinding;
use crate::models::character::model::Character;
use crate::models::character::{self, keybinding};
use crate::net::packet::handler::change_keymap::reader::ChangeKeymapReader;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

pub struct ChangeKeymapStore {
    pub char: Character,
    pub binds: Vec<Keybinding>,
}

impl ChangeKeymapStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_change_keymap(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &ChangeKeymapReader,
    ) -> Result<Self, NetworkError> {
        let char_id = session
            .char_id
            .ok_or(SessionError::NoCharacterSelected(session.id))?;
        let char = character::query::get_character_by_id(state, &char_id).await?;
        let new_binds: Vec<NewKeybinding> = izip!(read.keys, read.types, read.model)
            .map(|(key, bind_type, action): (i32, i16, i32)| NewKeybinding {
                char_id: char.id.clone(),
                key: key.clone(),
                bind_type: bind_type.clone(),
                action: action.clone(),
            })
            .collect();
        let binds = keybinding::query::update_keybindings(state, &new_binds).await?;
        Ok(Self {
            char: char.clone(),
            binds: binds.clone(),
        })
    }
}
