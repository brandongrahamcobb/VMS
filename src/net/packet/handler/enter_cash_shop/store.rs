use crate::constants::CASH_SHOP_MAP_ID;
use crate::models::account::model::AccountModel;
use crate::models::character::model::Character;
use crate::models::map;
use crate::models::map::model::MapModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::enter_cash_shop::reader::EnterCashShopReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct EnterCashShopStore {
    pub acc_model: AccountModel,
    pub char: Character,
    pub map_model: MapModel,
}

impl EnterCashShopStore {
    pub async fn store_enter_cash_shop(
        state: &SharedState,
        session: Session,
        reader: EnterCashShopReader,
    ) -> Result<Self, NetworkError> {
        let acc_model = session.acc.model.clone();
        let char = session.char.clone();
        let map_model = map::service::get_map_model_by_id(CASH_SHOP_MAP_ID)?;
        Ok(Self {
            acc_model,
            char,
            map_model,
        })
    }
}
