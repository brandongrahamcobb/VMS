use std::time::SystemTime;

use crate::models::character::equipment_set::android::model::{
    AndroidEquipmentSet, AndroidEquipmentSetModel,
};
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
        hat: wz::equip::service::get_equip_by_id(state, model.hat_id).await?,
        face: wz::equip::service::get_equip_by_id(state, model.face_id).await?,
        top: wz::equip::service::get_equip_by_id(state, model.top_id).await?,
        bottom: wz::equip::service::get_equip_by_id(state, model.bottom_id).await?,
        gloves: wz::equip::service::get_equip_by_id(state, model.gloves_id).await?,
        cape: wz::equip::service::get_equip_by_id(state, model.cape_id).await?,
    })
}

impl AndroidEquipmentSetModel {
    pub fn new() -> Self {
        Self {
            char_id: -1,
            hat_id: -1,
            face_id: -1,
            top_id: -1,
            bottom_id: -1,
            gloves_id: -1,
            cape_id: -1,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

impl AndroidEquipmentSet {
    pub fn new() -> Self {
        Self {
            model: AndroidEquipmentSetModel::new(),
            hat: Equip::new(),
            face: Equip::new(),
            top: Equip::new(),
            bottom: Equip::new(),
            gloves: Equip::new(),
            cape: Equip::new(),
        }
    }
}
