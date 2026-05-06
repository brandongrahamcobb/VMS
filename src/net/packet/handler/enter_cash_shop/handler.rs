use crate::constants::CASH_SHOP_MAP_ID;
use crate::models::account::model::Account;
use crate::models::character::equipment_set;
use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::model::Character;
use crate::models::{account, character};
use crate::net::action::model::{Action, PlayerAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::enter_cash_shop::store::EnterCashShopStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
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
        state: &SharedState,
        session: &Session,
        _packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, NetworkError> {
        let store = EnterCashShopStore::new()
            .store_enter_cash_shop(state, session)
            .await?;
        let result = self
            .build_enter_cash_shop_result(state, session, &store)
            .await?;
        Ok(result)
    }

    async fn build_enter_cash_shop_result(
        state: &SharedState,
        session: &Session,
        store: &EnterCashShopStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult<PlayerAction> = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_enter_cash_shop_handler_packet(
                state,
                &store.acc,
                &store.char,
                &store.regular_equips,
                &store.cash_equips,
            )
            .await?
            .finish();
        result.add_action(Action::Local {
            packet: packet.clone(),
        });
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(&store.char.id)?
            .finish();
        result.add_action(Action::Player(PlayerAction::ExitMap {
            session: session.clone(),
            packet: packet.clone(),
            source_world_id: Some(store.world.id),
            source_channel_id: Some(store.channel.id),
            source_map_id: Some(store.map.id),
        }));
        Ok(result)
    }
}
