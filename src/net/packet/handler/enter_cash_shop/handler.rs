use crate::constants::CASH_SHOP_MAP_ID;
use crate::models::account::model::Account;
use crate::models::character::equipment_set;
use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::model::Character;
use crate::models::{account, character};
use crate::net::action::{Action, PlayerAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::enter_cash_shop::reader::EnterCashShopReader;
use crate::net::packet::handler::enter_cash_shop::store::EnterCashShopStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct EnterCashShopHandler;

impl EnterCashShopHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        _packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: EnterCashShopReader =
            EnterCashShopReader::new().read_enter_cash_shop_packet(packet)?;
        let store: EnterCashShopStore = EnterCashShopStore::new()
            .store_enter_cash_shop(state, session, &reader)
            .await?;
        let result: HandlerResult = self.build_enter_cash_shop_result(&store)?;
        Ok(result)
    }

    fn build_enter_cash_shop_result(
        &self,
        store: &EnterCashShopStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        result.add_action(Action::SetMap {
            map: store.map.clone(),
            scope: Scope::Local,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_enter_cash_shop_handler_packet(
                &store.acc,
                &store.char,
                &store.regular_equips,
                &store.cash_equips,
            )
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(&store.char.id)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map,
        })?;
        Ok(result)
    }
}
