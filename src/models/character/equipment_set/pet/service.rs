use crate::models::character::equipment_set::pet::model::{PetEquipmentSet, PetEquipmentSetModel};
use crate::models::error::ModelError;
use crate::models::wz;
use crate::runtime::state::SharedState;

pub async fn get_pet_equipment_set_from_model(
    state: &SharedState,
    model: PetEquipmentSetModel,
) -> Result<PetEquipmentSet, ModelError> {
    Ok(PetEquipmentSet {
        model,
        accessory_one: wz::equip::query::get_equip_by_id(state, model.accessory_one_id).await?,
        accessory_two: wz::equip::query::get_equip_by_id(state, model.accessory_two_id).await?,
        accessory_three: wz::equip::query::get_equip_by_id(state, model.accessory_three_id).await?,
    })
}
