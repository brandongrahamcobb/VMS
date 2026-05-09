use crate::models::character::equipment_set::error::EquipmentSetError;
use crate::models::character::equipment_set::pet::error::PetEquipmentSetModelError;
use crate::models::character::equipment_set::pet::model::{PetEquipmentSet, PetEquipmentSetModel};
use crate::models::character::error::CharacterError;
use crate::models::error::ModelError;
use crate::models::item::equip;
use crate::runtime::state::SharedState;

impl PetEquipmentSetModel {
    pub async fn load(&self, state: &SharedState) -> Result<PetEquipmentSet, ModelError> {
        Ok(PetEquipmentSet {
            model: self.clone(),
            accessory_one: equip::service::resolve_equip(state, self.accessory_one_id).await?,
            accessory_two: equip::service::resolve_equip(state, self.accessory_two_id).await?,
            accessory_three: equip::service::resolve_equip(state, self.accessory_three_id).await?,
        })
    }

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
