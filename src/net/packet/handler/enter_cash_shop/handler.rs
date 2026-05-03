use crate::db::error::DatabaseError;
use crate::models::channel::error::ChannelError;
use crate::models::character::error::CharacterError;
use crate::models::error::ModelError;
use crate::models::{account, character};
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct EnterCashShopHandler;

impl EnterCashShopHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        session: Session,
        _packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let acc = account::query::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let char_id = acc
            .selected_char_id
            .ok_or(CharacterError::NotSelected(acc_id))
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let channel_id = acc
            .selected_channel_id
            .ok_or(ChannelError::NotSelected(acc_id))
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let char = character::query::get_character_by_id(state.clone(), char_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let regular_equips =
            character::equipment_set::query::get_regular_equipment_set_by_character_id(
                state.clone(),
                char_id,
            )
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let cash_equips = character::equipment_set::query::get_cash_equipment_set_by_character_id(
            state.clone(),
            char_id,
        )
        .await
        .map_err(DatabaseError::from)
        .map_err(NetworkError::from)?;
        let mut result: HandlerResult<ChannelAction> = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_enter_cash_shop_handler_packet(
                state.clone(),
                channel_id,
                &acc,
                &char,
                &regular_equips,
                &cash_equips,
            )
            .await?
            .finish();
        let action = ChannelAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
