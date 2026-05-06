use crate::config::settings;
use crate::models::character::model::Character;
use crate::models::{account, character, world};
use crate::net::action::model::{Action, LoginAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_chars;
use crate::net::packet::handler::list_chars::store::ListCharsStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ListCharsHandler;

impl ListCharsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let read = ListCharsRead::new().read_list_chars_packet(packet)?;
        let store = ListCharsStore::new().store_list_chars(state, session, &read)?;
        let result = self.build_list_chars_result(state, session, &store).await?;
        Ok(result)
    }
}

async fn complete_list_chars_handler(
    state: &SharedState,
    session: &Session,
    store: &ListCharsStore,
) -> Result<HandlerResult, NetworkError> {
    let mut result: HandlerResult = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_list_chars_handler_packet(
            state,
            &store.channel.id,
            &store.chars,
            &store.char_max,
            &store.pic_status,
        )
        .await?
        .finish();
    let action = Action::Local {
        session: session.clone(),
        packet: packet.clone(),
    };
    result.add_action(action);
    Ok(result)
}
