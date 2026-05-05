use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::model::Character;
use crate::models::character::{equipment_set, keybinding};
use crate::net::error::NetworkError;
use crate::runtime::state::SharedState;

pub async fn create_equips_on_join(
    state: &SharedState,
    char: &Character,
) -> Result<(RegularEquipmentSet, CashEquipmentSet), NetworkError> {
    let regular_equips =
        equipment_set::query::get_regular_equipment_set_by_character_id(state, &char.id).await?;
    let cash_equips =
        equipment_set::query::get_cash_equipment_set_by_character_id(state, &char.id).await?;
    Ok((regular_equips, cash_equips))
}

pub async fn create_keybindings_on_join(
    state: &SharedState,
    char: &Character,
) -> Result<Vec<Keybinding>, NetworkError> {
    let binds = keybinding::query::get_keybindings_by_character_id(state, &char.id).await?;
    let binds: Vec<Keybinding> = keybinding::service::normalize_keybindings(&binds, &char.id);
    Ok(binds)
}
