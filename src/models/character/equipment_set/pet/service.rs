use std::time::SystemTime;

use crate::models::character::equipment_set::pet::model::{PetEquipmentSet, PetEquipmentSetModel};
use crate::models::error::ModelError;
use crate::models::wz;
use crate::models::wz::equip::model::Equip;
use crate::runtime::state::SharedState;

pub async fn get_pet_equipment_set_from_model(
    state: &SharedState,
    model: PetEquipmentSetModel,
) -> Result<PetEquipmentSet, ModelError> {
    Ok(PetEquipmentSet {
        model: model.clone(),
        accessory_one: wz::equip::service::get_equip_by_id(state, model.accessory_one_id).await?,
        accessory_two: wz::equip::service::get_equip_by_id(state, model.accessory_two_id).await?,
        accessory_three: wz::equip::service::get_equip_by_id(state, model.accessory_three_id)
            .await?,
    })
}

impl PetEquipmentSetModel {
    pub fn new() -> Self {
        Self {
            char_id: -1,
            accessory_one_id: -1,
            accessory_two_id: -1,
            accessory_three_id: -1,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

impl PetEquipmentSet {
    pub fn new() -> Self {
        Self {
            model: PetEquipmentSetModel::new(),
            accessory_one: Equip::new(),
            accessory_two: Equip::new(),
            accessory_three: Equip::new(),
        }
    }
}
