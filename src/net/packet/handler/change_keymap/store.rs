use crate::models::character::keybinding;
use crate::models::character::keybinding::model::NewKeybindingInsert;
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_keymap::reader::ChangeKeymapReader;
use crate::runtime::session::Session;
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
        let new_binds: Vec<NewKeybindingInsert> = izip!(
            reader.keys.clone(),
            reader.types.clone(),
            reader.model.clone()
        )
        .map(
            |(key, bind_type, action): (i32, i16, i32)| NewKeybindingInsert {
                char_id: char.model.id,
                key,
                bind_type,
                action,
            },
        )
        .collect();
        keybinding::query::update_keybindings(state, new_binds.clone()).await?;
        return Ok(Self);
    }
}
