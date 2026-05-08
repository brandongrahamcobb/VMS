use crate::models::character::equipment_set::error::EquipmentSetError;
use crate::models::character::equipment_set::pet::error::PetEquipmentSetModelError;
use crate::models::character::equipment_set::pet::model::{
    NewPetEquipmentSetInsert, PetEquipmentSet, PetEquipmentSetModel,
};
use crate::models::character::error::CharacterError;
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
        accessory_one: wz::equip::service::resolve_equip(state, model.accessory_one_id).await?,
        accessory_two: wz::equip::service::resolve_equip(state, model.accessory_two_id).await?,
        accessory_three: wz::equip::service::resolve_equip(state, model.accessory_three_id).await?,
    })
}

impl NewPetEquipmentSetInsert {
    pub fn default(
        char_id: i32,
        accessory_one_id: Option<i32>,
        accessory_two_id: Option<i32>,
        accessory_three_id: Option<i32>,
    ) -> Self {
        Self {
            char_id,
            accessory_one_id,
            accessory_two_id,
            accessory_three_id,
        }
    }
}

impl PetEquipmentSet {
    pub fn new(
        model: PetEquipmentSetModel,
        accessory_one: Option<Equip>,
        accessory_two: Option<Equip>,
        accessory_three: Option<Equip>,
    ) -> Self {
        Self {
            model,
            accessory_one,
            accessory_two,
            accessory_three,
        }
    }
}

impl PetEquipmentSetModel {
    pub fn get_accessory_one_id(&self) -> Result<i32, ModelError> {
        if let Some(accessory_one_id) = self.accessory_one_id {
            Ok(accessory_one_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(PetEquipmentSetModelError::NoAccessoryOneId(self.char_id)),
            )))
        }
    }

    pub fn get_accessory_two_id(&self) -> Result<i32, ModelError> {
        if let Some(accessory_two_id) = self.accessory_two_id {
            Ok(accessory_two_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(PetEquipmentSetModelError::NoAccessoryTwoId(self.char_id)),
            )))
        }
    }

    pub fn get_accessory_three_id(&self) -> Result<i32, ModelError> {
        if let Some(accessory_three_id) = self.accessory_three_id {
            Ok(accessory_three_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(PetEquipmentSetModelError::NoAccessoryThreeId(
                    self.char_id,
                )),
            )))
        }
    }
}
