use crate::models::character::equipment_set::android::error::AndroidEquipmentSetModelError;
use crate::models::character::equipment_set::android::model::{
    AndroidEquipmentSet, AndroidEquipmentSetModel, NewAndroidEquipmentSetInsert,
};
use crate::models::character::equipment_set::error::EquipmentSetError;
use crate::models::character::error::CharacterError;
use crate::models::error::ModelError;
use crate::models::wz;
use crate::models::wz::equip::model::Equip;
use crate::runtime::state::SharedState;

pub async fn get_android_equipment_set_from_model(
    state: &SharedState,
    model: AndroidEquipmentSetModel,
) -> Result<AndroidEquipmentSet, ModelError> {
    Ok(AndroidEquipmentSet {
        model: model.clone(),
        hat: wz::equip::service::resolve_equip(state, model.hat_id).await?,
        face: wz::equip::service::resolve_equip(state, model.face_id).await?,
        top: wz::equip::service::resolve_equip(state, model.top_id).await?,
        bottom: wz::equip::service::resolve_equip(state, model.bottom_id).await?,
        gloves: wz::equip::service::resolve_equip(state, model.gloves_id).await?,
        cape: wz::equip::service::resolve_equip(state, model.cape_id).await?,
    })
}

impl NewAndroidEquipmentSetInsert {
    pub fn default(
        char_id: i32,
        bottom_id: Option<i32>,
        cape_id: Option<i32>,
        face_id: Option<i32>,
        gloves_id: Option<i32>,
        hat_id: Option<i32>,
        top_id: Option<i32>,
    ) -> Self {
        Self {
            char_id,
            bottom_id,
            cape_id,
            face_id,
            gloves_id,
            hat_id,
            top_id,
        }
    }
}

impl AndroidEquipmentSet {
    pub fn new(
        model: AndroidEquipmentSetModel,
        bottom: Option<Equip>,
        cape: Option<Equip>,
        face: Option<Equip>,
        gloves: Option<Equip>,
        hat: Option<Equip>,
        top: Option<Equip>,
    ) -> Self {
        Self {
            model,
            bottom,
            cape,
            face,
            gloves,
            hat,
            top,
        }
    }
}

impl AndroidEquipmentSetModel {
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
