use crate::models::character::keybinding;
use crate::models::character::keybinding::model::NewKeybinding;
use crate::net::action::model::PlayerAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_keymap;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

pub struct ChangeKeymapHandler;

impl ChangeKeymapHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, NetworkError> {
        let read = change_keymap::read::read_change_keymap_packet(packet)?;
        let char_id = session
            .char_id
            .ok_or(SessionError::NoCharacterSelected(session.id))?;
        let binds: Vec<NewKeybinding> = izip!(read.keys, read.types, read.model)
            .map(|(key, bind_type, action): (i32, i16, i32)| NewKeybinding {
                char_id: char_id,
                key,
                bind_type,
                action,
            })
            .collect();
        keybinding::query::update_keybindings(state, &binds).await?;
        let result = complete_change_keymap_handler()?;
        Ok(result)
    }
}

fn complete_change_keymap_handler() -> Result<HandlerResult<PlayerAction>, NetworkError> {
    let mut result: HandlerResult<PlayerAction> = HandlerResult::new();
    result.add_action(PlayerAction::Simple);
    Ok(result)
}
