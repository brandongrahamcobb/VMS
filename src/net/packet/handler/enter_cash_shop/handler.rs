use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::enter_cash_shop::reader::EnterCashShopReader;
use crate::net::packet::handler::enter_cash_shop::store::EnterCashShopStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct EnterCashShopHandler;

impl EnterCashShopHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: EnterCashShopReader = EnterCashShopReader::read_enter_cash_shop_packet(packet)?;
        let store: EnterCashShopStore =
            EnterCashShopStore::store_enter_cash_shop(state, session.clone(), reader).await?;
        let result: HandlerResult = self.build_enter_cash_shop_result(store.clone())?;
        Ok(result)
    }

    fn build_enter_cash_shop_result(
        &self,
        store: EnterCashShopStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        result.add_action(Action::Set(SetAction::SetMap {
            map: store.map.clone(),
            scope: Scope::Local,
        }))?;
        let packet: Packet = Packet::new_empty()
            .build_enter_cash_shop_handler_packet(store.acc.clone(), store.char.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(store.char.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        })?;
        Ok(result)
    }
}
