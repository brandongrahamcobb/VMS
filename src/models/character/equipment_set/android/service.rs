use crate::models::character::equipment_set::android::error::AndroidEquipmentSetModelError;
use crate::models::character::equipment_set::android::model::{
    AndroidEquipmentSet, AndroidEquipmentSetModel,
};
use crate::models::character::equipment_set::error::EquipmentSetError;
use crate::models::character::error::CharacterError;
use crate::models::error::ModelError;
use crate::models::item::equip;
use crate::runtime::state::SharedState;

impl AndroidEquipmentSetModel {
    pub async fn load(&self, state: &SharedState) -> Result<AndroidEquipmentSet, ModelError> {
        Ok(AndroidEquipmentSet {
            model: self.clone(),
            hat: equip::service::resolve_equip(state, self.hat_id).await?,
            face: equip::service::resolve_equip(state, self.face_id).await?,
            top: equip::service::resolve_equip(state, self.top_id).await?,
            bottom: equip::service::resolve_equip(state, self.bottom_id).await?,
            gloves: equip::service::resolve_equip(state, self.gloves_id).await?,
            cape: equip::service::resolve_equip(state, self.cape_id).await?,
        })
    }

    pub fn get_bottom_id(&self) -> Result<i32, ModelError> {
        if let Some(bottom_id) = self.bottom_id {
            Ok(bottom_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(AndroidEquipmentSetModelError::NoBottomId(self.char_id)),
            )))
        }
    }

    pub fn get_cape_id(&self) -> Result<i32, ModelError> {
        if let Some(cape_id) = self.cape_id {
            Ok(cape_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(AndroidEquipmentSetModelError::NoCapeId(self.char_id)),
            )))
        }
    }

    pub fn get_face_id(&self) -> Result<i32, ModelError> {
        if let Some(face_id) = self.face_id {
            Ok(face_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(AndroidEquipmentSetModelError::NoFaceId(self.char_id)),
            )))
        }
    }

    pub fn get_gloves_id(&self) -> Result<i32, ModelError> {
        if let Some(gloves_id) = self.gloves_id {
            Ok(gloves_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(AndroidEquipmentSetModelError::NoGlovesId(self.char_id)),
            )))
        }
    }

    pub fn get_hat_id(&self) -> Result<i32, ModelError> {
        if let Some(hat_id) = self.hat_id {
            Ok(hat_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(AndroidEquipmentSetModelError::NoHatId(self.char_id)),
            )))
        }
    }

    pub fn get_top_id(&self) -> Result<i32, ModelError> {
        if let Some(top_id) = self.top_id {
            Ok(top_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(AndroidEquipmentSetModelError::NoTopId(self.char_id)),
            )))
        }
    }
}
