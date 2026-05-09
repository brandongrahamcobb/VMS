use std::time::SystemTime;

use crate::models::character::keybinding;
use crate::models::character::keybinding::model::KeybindingModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_keymap::reader::ChangeKeymapReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

#[derive(Clone)]
pub struct ChangeKeymapStore;

impl ChangeKeymapStore {
    pub async fn store_change_keymap(
        state: &SharedState,
        session: Session,
        reader: ChangeKeymapReader,
    ) -> Result<Self, NetworkError> {
        let char = session.get_char()?;
        let char_id = char.model.get_id()?;
        let new_binds: Vec<KeybindingModel> = izip!(
            reader.keys.clone(),
            reader.types.clone(),
            reader.model.clone()
        )
        .map(
            |(key, bind_type, action): (i32, i16, i32)| KeybindingModel {
                char_id,
                key,
                bind_type,
                action,
                created_at: None,
                updated_at: SystemTime::now(),
            },
        )
        .collect();
        keybinding::query::setters::update_keybindings(state, new_binds.clone()).await?;
        return Ok(Self);
    }
}
