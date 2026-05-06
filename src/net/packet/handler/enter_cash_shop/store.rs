use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::{account, character};
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct EnterCashShopStore {
    pub acc: Account,
    pub char: Character,
    pub regular_equips: RegularEquipmentSet,
    pub cash_equips: CashEquipmentSet,
}

impl EnterCashShopStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_enter_cash_shop(
        &self,
        state: &SharedState,
        session: &Session,
    ) -> Result<Self, NetworkError> {
        let acc_id = session.acc_id;
        let acc = account::query::get_account_by_id(state, &acc_id).await?;
        let char_id = session
            .char_id
            .ok_or(SessionError::NoCharacterSelected(session.id))?;
        let char = character::query::get_character_by_id(state, &char_id).await?;
        let regular_equips =
            equipment_set::query::get_regular_equipment_set_by_character_id(state, &char_id)
                .await?;
        let cash_equips =
            equipment_set::query::get_cash_equipment_set_by_character_id(state, &char_id).await?;
        Ok(Self {
            acc: acc.clone(),
            char: char.clone(),
            regular_equips: regular_equips.clone(),
            cash_equips: cash_equips.clone(),
        })
    }
}
