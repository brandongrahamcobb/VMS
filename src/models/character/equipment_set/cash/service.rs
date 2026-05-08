use std::time::SystemTime;

use crate::models::character::equipment_set::cash::model::{
    CashEquipmentSet, CashEquipmentSetModel,
};
use crate::models::error::ModelError;
use crate::models::wz;
use crate::models::wz::equip::model::Equip;
use crate::runtime::state::SharedState;

pub async fn get_cash_equipment_set_from_model(
    state: &SharedState,
    model: CashEquipmentSetModel,
) -> Result<CashEquipmentSet, ModelError> {
    Ok(CashEquipmentSet {
        model: model.clone(),
        hat: wz::equip::service::get_equip_by_id(state, model.hat_id).await?,
        face_acc: wz::equip::service::get_equip_by_id(state, model.face_acc_id).await?,
        eye_acc: wz::equip::service::get_equip_by_id(state, model.eye_acc_id).await?,
        ear_acc: wz::equip::service::get_equip_by_id(state, model.ear_acc_id).await?,
        top: wz::equip::service::get_equip_by_id(state, model.top_id).await?,
        bottom: wz::equip::service::get_equip_by_id(state, model.bottom_id).await?,
        shoes: wz::equip::service::get_equip_by_id(state, model.shoes_id).await?,
        gloves: wz::equip::service::get_equip_by_id(state, model.gloves_id).await?,
        cape: wz::equip::service::get_equip_by_id(state, model.cape_id).await?,
        weapon: wz::equip::service::get_equip_by_id(state, model.weapon_id).await?,
        ring_one: wz::equip::service::get_equip_by_id(state, model.ring_one_id).await?,
        ring_two: wz::equip::service::get_equip_by_id(state, model.ring_two_id).await?,
        ring_three: wz::equip::service::get_equip_by_id(state, model.ring_three_id).await?,
        ring_four: wz::equip::service::get_equip_by_id(state, model.ring_four_id).await?,
        pendant: wz::equip::service::get_equip_by_id(state, model.pendant_id).await?,
        belt: wz::equip::service::get_equip_by_id(state, model.belt_id).await?,
        shoulder: wz::equip::service::get_equip_by_id(state, model.shoulder_id).await?,
        subweapon: wz::equip::service::get_equip_by_id(state, model.subweapon_id).await?,
        hair: wz::equip::service::get_equip_by_id(state, model.hair_id).await?,
    })
}

impl CashEquipmentSetModel {
    pub fn new() -> Self {
        Self {
            char_id: -1,
            hat_id: -1,
            face_acc_id: -1,
            eye_acc_id: -1,
            ear_acc_id: -1,
            top_id: -1,
            bottom_id: -1,
            shoes_id: -1,
            gloves_id: -1,
            cape_id: -1,
            weapon_id: -1,
            ring_one_id: -1,
            ring_two_id: -1,
            ring_three_id: -1,
            ring_four_id: -1,
            pendant_id: -1,
            belt_id: -1,
            shoulder_id: -1,
            subweapon_id: -1,
            hair_id: -1,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

impl CashEquipmentSet {
    pub fn new() -> Self {
        Self {
            model: CashEquipmentSetModel::new(),
            hat: Equip::new(),
            face_acc: Equip::new(),
            eye_acc: Equip::new(),
            ear_acc: Equip::new(),
            top: Equip::new(),
            bottom: Equip::new(),
            shoes: Equip::new(),
            gloves: Equip::new(),
            cape: Equip::new(),
            weapon: Equip::new(),
            ring_one: Equip::new(),
            ring_two: Equip::new(),
            ring_three: Equip::new(),
            ring_four: Equip::new(),
            pendant: Equip::new(),
            belt: Equip::new(),
            shoulder: Equip::new(),
            subweapon: Equip::new(),
            hair: Equip::new(),
        }
    }
}
