use crate::constants::CASH_SHOP_MAP_ID;
use crate::models::account::model::Account;
use crate::models::character::model::Character;
use crate::models::shroom::map;
use crate::models::shroom::map::model::Map;
use crate::net::error::NetworkError;
use crate::net::packet::handler::enter_cash_shop::reader::EnterCashShopReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct EnterCashShopStore {
    pub acc: Account,
    pub char: Character,
    pub map: Map,
}

impl EnterCashShopStore {
    pub async fn store_enter_cash_shop(
        state: &SharedState,
        session: Session,
        reader: EnterCashShopReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(state);
        std::hint::black_box(reader);
        let acc = session.get_acc()?;
        let char = session.get_char()?;
        let map = map::service::get_map_by_id(CASH_SHOP_MAP_ID)?;
        Ok(Self { acc, char, map })
    }
}
