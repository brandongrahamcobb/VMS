use crate::config::settings;
use crate::models::character::model::Character;
use crate::models::{account, character, world};
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::list_chars;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ListCharsHandler;

impl ListCharsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let read = list_chars::read::read_list_chars_packet(packet)?;
        {
            let state = state.lock().await;
            state.sessions.update(session.id, |s| {
                s.channel_id = Some(read.channel_id);
                s.world_id = Some(read.world_id);
            });
        }
        let acc_id = session.acc_id;
        let acc = account::query::get_account_by_id(state.clone(), &acc_id).await?;
        let chars = character::query::get_characters_by_account_id_and_world_id(
            state.clone(),
            &acc_id,
            &read.world_id,
        )
        .await?;
        let default_char_max = settings::get_char_max()?;
        let char_max = world::query::get_character_max_by_account_and_world_id(
            state.clone(),
            &acc_id,
            &read.world_id,
        )
        .await
        .unwrap_or(default_char_max as i16);
        let mut pic_status: i8 = 0;
        let use_pic = settings::get_pic_required()?;
        if let Some(_pic) = acc.pic {
            pic_status = if use_pic { 1 } else { 2 };
        }
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_list_chars_handler_packet(
                state.clone(),
                &read.channel_id,
                chars.clone(),
                &char_max,
                &pic_status,
            )
            .await?
            .finish();
        let action = LoginAction::SendPacket { packet };
        result.add_action(action);
        Ok(result)
    }
}

async fn complete_list_chars_handler(
    state: SharedState,
    channel_id: &i8,
    chars: Vec<Character>,
    char_max: &i16,
    pic_status: &i8,
) -> Result<HandlerResult<LoginAction>, NetworkError> {
    let mut result: HandlerResult<LoginAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_list_chars_handler_packet(
            state.clone(),
            channel_id,
            chars.clone(),
            char_max,
            pic_status,
        )
        .await?
        .finish();
    let action = LoginAction::SendPacket { packet };
    result.add_action(action);
    Ok(result)
}
