use crate::models::character::equipment_set::android::model::{
    AndroidEquipmentSet, AndroidEquipmentSetModel,
};
use crate::models::error::ModelError;
use crate::models::wz;
use crate::runtime::state::SharedState;

pub async fn get_android_equipment_set_from_model(
    state: &SharedState,
    model: AndroidEquipmentSetModel,
) -> Result<AndroidEquipmentSet, ModelError> {
    Ok(AndroidEquipmentSet {
        model,
        hat: wz::equip::query::get_equip_by_id(state, model.hat_id).await?,
        face: wz::equip::query::get_equip_by_id(state, model.face_id).await?,
        top: wz::equip::query::get_equip_by_id(state, model.top_id).await?,
        bottom: wz::equip::query::get_equip_by_id(state, model.bottom_id).await?,
        gloves: wz::equip::query::get_equip_by_id(state, model.gloves_id).await?,
        cape: wz::equip::query::get_equip_by_id(state, model.cape_id).await?,
    })
}
