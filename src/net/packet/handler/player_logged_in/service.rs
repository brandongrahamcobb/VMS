use crate::models::character::equipment_set::cash::model::CashEquipmentSetModel;
use crate::models::character::equipment_set::regular::model::RegularEquipmentSetModel;
use crate::models::character::keybinding::model::KeybindingModel;
use crate::models::character::model::CharacterModel;
use crate::models::character::{equipment_set, keybinding};
use crate::net::error::NetworkError;
use crate::runtime::state::SharedState;

pub async fn create_equips_on_join(
    state: &SharedState,
    char: &CharacterModel,
) -> Result<(RegularEquipmentSetModel, CashEquipmentSetModel), NetworkError> {
    let regular_equips =
        equipment_set::regular::query::get_regular_equipment_set_by_character_id(state, char.id)
            .await?;
    let cash_equips =
        equipment_set::cash::query::get_cash_equipment_set_by_character_id(state, char.id).await?;
    Ok((regular_equips, cash_equips))
}

pub async fn create_keybindings_on_join(
    state: &SharedState,
    char: &CharacterModel,
) -> Result<Vec<KeybindingModel>, NetworkError> {
    let binds = keybinding::query::get_keybindings_by_character_id(state, char.id).await?;
    let binds: Vec<KeybindingModel> = keybinding::service::normalize_keybindings(binds, char.id);
    Ok(binds)
}
