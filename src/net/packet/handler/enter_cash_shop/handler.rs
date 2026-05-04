use crate::models::account::model::Account;
use crate::models::character::equipment_set;
use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::model::Character;
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
        let acc_id = session.acc_id;
        let acc = account::query::get_account_by_id(state.clone(), &acc_id).await?;
        let char_id = session
            .char_id
            .ok_or(SessionError::NoCharacterSelected(session.id))?;
        let char = character::query::get_character_by_id(state.clone(), &char_id).await?;
        let regular_equips = equipment_set::query::get_regular_equipment_set_by_character_id(
            state.clone(),
            &char_id,
        )
        .await?;
        let cash_equips =
            equipment_set::query::get_cash_equipment_set_by_character_id(state.clone(), &char_id)
                .await?;
        let result = complete_enter_cash_shop_handler(
            state.clone(),
            &acc,
            &char,
            &regular_equips,
            &cash_equips,
        )
        .await?;
        Ok(result)
    }
}

async fn complete_enter_cash_shop_handler(
    state: SharedState,
    acc: &Account,
    char: &Character,
    regular_equips: &RegularEquipmentSet,
    cash_equips: &CashEquipmentSet,
) -> Result<HandlerResult<ChannelAction>, NetworkError> {
    let mut result: HandlerResult<ChannelAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_enter_cash_shop_handler_packet(state.clone(), acc, char, regular_equips, cash_equips)
        .await?
        .finish();
    result.add_action(ChannelAction::SendPacket { packet: packet.clone() });
    Ok(result)
}
