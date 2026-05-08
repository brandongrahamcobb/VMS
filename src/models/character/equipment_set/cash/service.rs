use crate::models::character::equipment_set::cash::error::CashEquipmentSetModelError;
use crate::models::character::equipment_set::cash::model::{
    CashEquipmentSet, CashEquipmentSetModel, NewCashEquipmentSetInsert,
};
use crate::models::character::equipment_set::error::EquipmentSetError;
use crate::models::character::error::CharacterError;
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
        hat: wz::equip::service::resolve_equip(state, model.hat_id).await?,
        face_acc: wz::equip::service::resolve_equip(state, model.face_acc_id).await?,
        eye_acc: wz::equip::service::resolve_equip(state, model.eye_acc_id).await?,
        ear_acc: wz::equip::service::resolve_equip(state, model.ear_acc_id).await?,
        top: wz::equip::service::resolve_equip(state, model.top_id).await?,
        bottom: wz::equip::service::resolve_equip(state, model.bottom_id).await?,
        shoes: wz::equip::service::resolve_equip(state, model.shoes_id).await?,
        gloves: wz::equip::service::resolve_equip(state, model.gloves_id).await?,
        cape: wz::equip::service::resolve_equip(state, model.cape_id).await?,
        weapon: wz::equip::service::resolve_equip(state, model.weapon_id).await?,
        ring_one: wz::equip::service::resolve_equip(state, model.ring_one_id).await?,
        ring_two: wz::equip::service::resolve_equip(state, model.ring_two_id).await?,
        ring_three: wz::equip::service::resolve_equip(state, model.ring_three_id).await?,
        ring_four: wz::equip::service::resolve_equip(state, model.ring_four_id).await?,
        pendant: wz::equip::service::resolve_equip(state, model.pendant_id).await?,
        belt: wz::equip::service::resolve_equip(state, model.belt_id).await?,
        shoulder: wz::equip::service::resolve_equip(state, model.shoulder_id).await?,
        subweapon: wz::equip::service::resolve_equip(state, model.subweapon_id).await?,
        hair: wz::equip::service::resolve_equip(state, model.hair_id).await?,
    })
}

impl NewCashEquipmentSetInsert {
    pub fn default(
        char_id: i32,
        belt_id: Option<i32>,
        bottom_id: Option<i32>,
        cape_id: Option<i32>,
        ear_acc_id: Option<i32>,
        eye_acc_id: Option<i32>,
        face_acc_id: Option<i32>,
        gloves_id: Option<i32>,
        hair_id: Option<i32>,
        hat_id: Option<i32>,
        pendant_id: Option<i32>,
        ring_four_id: Option<i32>,
        ring_one_id: Option<i32>,
        ring_three_id: Option<i32>,
        ring_two_id: Option<i32>,
        shoes_id: Option<i32>,
        shoulder_id: Option<i32>,
        subweapon_id: Option<i32>,
        top_id: Option<i32>,
        weapon_id: Option<i32>,
    ) -> Self {
        Self {
            char_id,
            belt_id,
            bottom_id,
            cape_id,
            ear_acc_id,
            eye_acc_id,
            face_acc_id,
            gloves_id,
            hair_id,
            hat_id,
            pendant_id,
            ring_four_id,
            ring_one_id,
            ring_three_id,
            ring_two_id,
            shoes_id,
            shoulder_id,
            subweapon_id,
            top_id,
            weapon_id,
        }
    }
}

impl CashEquipmentSet {
    pub fn new(
        model: CashEquipmentSetModel,
        belt: Option<Equip>,
        bottom: Option<Equip>,
        cape: Option<Equip>,
        ear_acc: Option<Equip>,
        eye_acc: Option<Equip>,
        face_acc: Option<Equip>,
        gloves: Option<Equip>,
        hair: Option<Equip>,
        hat: Option<Equip>,
        pendant: Option<Equip>,
        ring_four: Option<Equip>,
        ring_one: Option<Equip>,
        ring_three: Option<Equip>,
        ring_two: Option<Equip>,
        shoes: Option<Equip>,
        shoulder: Option<Equip>,
        subweapon: Option<Equip>,
        top: Option<Equip>,
        weapon: Option<Equip>,
    ) -> Self {
        Self {
            model,
            hat,
            face_acc,
            eye_acc,
            ear_acc,
            top,
            bottom,
            shoes,
            gloves,
            cape,
            weapon,
            ring_one,
            ring_two,
            ring_three,
            ring_four,
            pendant,
            belt,
            shoulder,
            subweapon,
            hair,
        }
    }
}

impl CashEquipmentSetModel {
    pub fn get_belt_id(&self) -> Result<i32, ModelError> {
        if let Some(belt_id) = self.belt_id {
            Ok(belt_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoBeltId(self.char_id)),
            )))
        }
    }

    pub fn get_bottom_id(&self) -> Result<i32, ModelError> {
        if let Some(bottom_id) = self.bottom_id {
            Ok(bottom_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoBottomId(self.char_id)),
            )))
        }
    }

    pub fn get_cape_id(&self) -> Result<i32, ModelError> {
        if let Some(cape_id) = self.cape_id {
            Ok(cape_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoCapeId(self.char_id)),
            )))
        }
    }

    pub fn get_ear_acc_id(&self) -> Result<i32, ModelError> {
        if let Some(ear_acc_id) = self.ear_acc_id {
            Ok(ear_acc_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoEarAccId(self.char_id)),
            )))
        }
    }

    pub fn get_eye_acc_id(&self) -> Result<i32, ModelError> {
        if let Some(eye_acc_id) = self.eye_acc_id {
            Ok(eye_acc_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoEyeAccId(self.char_id)),
            )))
        }
    }

    pub fn get_face_acc_id(&self) -> Result<i32, ModelError> {
        if let Some(face_acc_id) = self.face_acc_id {
            Ok(face_acc_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoFaceAccId(self.char_id)),
            )))
        }
    }

    pub fn get_gloves_id(&self) -> Result<i32, ModelError> {
        if let Some(gloves_id) = self.gloves_id {
            Ok(gloves_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoGlovesId(self.char_id)),
            )))
        }
    }

    pub fn get_hair_id(&self) -> Result<i32, ModelError> {
        if let Some(hair_id) = self.hair_id {
            Ok(hair_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoHairId(self.char_id)),
            )))
        }
    }

    pub fn get_hat_id(&self) -> Result<i32, ModelError> {
        if let Some(hat_id) = self.hat_id {
            Ok(hat_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoHatId(self.char_id)),
            )))
        }
    }

    pub fn get_pendant_id(&self) -> Result<i32, ModelError> {
        if let Some(pendant_id) = self.pendant_id {
            Ok(pendant_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoPendantId(self.char_id)),
            )))
        }
    }

    pub fn get_ring_four_id(&self) -> Result<i32, ModelError> {
        if let Some(ring_four_id) = self.ring_four_id {
            Ok(ring_four_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoRingFourId(self.char_id)),
            )))
        }
    }

    pub fn get_ring_one_id(&self) -> Result<i32, ModelError> {
        if let Some(ring_one_id) = self.ring_one_id {
            Ok(ring_one_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoRingOneId(self.char_id)),
            )))
        }
    }

    pub fn get_ring_three_id(&self) -> Result<i32, ModelError> {
        if let Some(ring_three_id) = self.ring_three_id {
            Ok(ring_three_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoRingThreeId(self.char_id)),
            )))
        }
    }

    pub fn get_ring_two_id(&self) -> Result<i32, ModelError> {
        if let Some(ring_two_id) = self.ring_two_id {
            Ok(ring_two_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoRingTwoId(self.char_id)),
            )))
        }
    }

    pub fn get_shoes_id(&self) -> Result<i32, ModelError> {
        if let Some(shoes_id) = self.shoes_id {
            Ok(shoes_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoShoesId(self.char_id)),
            )))
        }
    }

    pub fn get_shoulder_id(&self) -> Result<i32, ModelError> {
        if let Some(shoulder_id) = self.shoulder_id {
            Ok(shoulder_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoShoulderId(self.char_id)),
            )))
        }
    }

    pub fn get_subweapon_id(&self) -> Result<i32, ModelError> {
        if let Some(subweapon_id) = self.subweapon_id {
            Ok(subweapon_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoSubweaponId(self.char_id)),
            )))
        }
    }

    pub fn get_top_id(&self) -> Result<i32, ModelError> {
        if let Some(top_id) = self.top_id {
            Ok(top_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoTopId(self.char_id)),
            )))
        }
    }

    pub fn get_weapon_id(&self) -> Result<i32, ModelError> {
        if let Some(weapon_id) = self.weapon_id {
            Ok(weapon_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(CashEquipmentSetModelError::NoWeaponId(self.char_id)),
            )))
        }
    }
}
