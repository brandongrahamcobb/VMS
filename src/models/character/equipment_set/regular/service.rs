use crate::models::character::equipment_set::error::EquipmentSetError;
use crate::models::character::equipment_set::regular::error::RegularEquipmentSetModelError;
use crate::models::character::equipment_set::regular::model::{
    NewRegularEquipmentSetInsert, RegularEquipmentSet, RegularEquipmentSetModel,
};
use crate::models::character::error::CharacterError;
use crate::models::error::ModelError;
use crate::models::wz;
use crate::models::wz::equip::model::Equip;
use crate::runtime::state::SharedState;

impl NewRegularEquipmentSetInsert {
    pub fn default(
        char_id: i32,
        android_id: Option<i32>,
        badge_id: Option<i32>,
        belt_id: Option<i32>,
        book_id: Option<i32>,
        bottom_id: Option<i32>,
        cape_id: Option<i32>,
        ear_acc_id: Option<i32>,
        emblem_id: Option<i32>,
        eye_acc_id: Option<i32>,
        face_acc_id: Option<i32>,
        gloves_id: Option<i32>,
        hat_id: Option<i32>,
        heart_id: Option<i32>,
        medal_id: Option<i32>,
        pendant_one_id: Option<i32>,
        pendant_two_id: Option<i32>,
        pocket_id: Option<i32>,
        ring_four_id: Option<i32>,
        ring_one_id: Option<i32>,
        ring_three_id: Option<i32>,
        ring_two_id: Option<i32>,
        saddle_id: Option<i32>,
        shield_id: Option<i32>,
        shoes_id: Option<i32>,
        shoulder_id: Option<i32>,
        subweapon_id: Option<i32>,
        tamed_mob_id: Option<i32>,
        top_id: Option<i32>,
        weapon_id: Option<i32>,
    ) -> Self {
        Self {
            android_id,
            badge_id,
            belt_id,
            book_id,
            bottom_id,
            cape_id,
            char_id,
            ear_acc_id,
            emblem_id,
            eye_acc_id,
            face_acc_id,
            gloves_id,
            hat_id,
            heart_id,
            medal_id,
            pendant_one_id,
            pendant_two_id,
            pocket_id,
            ring_four_id,
            ring_one_id,
            ring_three_id,
            ring_two_id,
            saddle_id,
            shield_id,
            shoes_id,
            shoulder_id,
            subweapon_id,
            tamed_mob_id,
            top_id,
            weapon_id,
        }
    }
}

impl RegularEquipmentSet {
    pub fn new(
        model: RegularEquipmentSetModel,
        android: Option<Equip>,
        badge: Option<Equip>,
        belt: Option<Equip>,
        book: Option<Equip>,
        bottom: Option<Equip>,
        cape: Option<Equip>,
        ear_acc: Option<Equip>,
        emblem: Option<Equip>,
        eye_acc: Option<Equip>,
        face_acc: Option<Equip>,
        gloves: Option<Equip>,
        hat: Option<Equip>,
        heart: Option<Equip>,
        medal: Option<Equip>,
        pendant_one: Option<Equip>,
        pendant_two: Option<Equip>,
        pocket: Option<Equip>,
        ring_four: Option<Equip>,
        ring_one: Option<Equip>,
        ring_three: Option<Equip>,
        ring_two: Option<Equip>,
        saddle: Option<Equip>,
        shield: Option<Equip>,
        shoes: Option<Equip>,
        shoulder: Option<Equip>,
        subweapon: Option<Equip>,
        tamed_mob: Option<Equip>,
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
            shield,
            weapon,
            ring_one,
            ring_two,
            ring_three,
            ring_four,
            pendant_one,
            tamed_mob,
            saddle,
            medal,
            belt,
            pocket,
            book,
            pendant_two,
            shoulder,
            android,
            emblem,
            badge,
            subweapon,
            heart,
        }
    }
}

pub async fn get_regular_equipment_set_from_model(
    state: &SharedState,
    model: RegularEquipmentSetModel,
) -> Result<RegularEquipmentSet, ModelError> {
    Ok(RegularEquipmentSet {
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
        shield: wz::equip::service::resolve_equip(state, model.shield_id).await?,
        weapon: wz::equip::service::resolve_equip(state, model.weapon_id).await?,
        ring_one: wz::equip::service::resolve_equip(state, model.ring_one_id).await?,
        ring_two: wz::equip::service::resolve_equip(state, model.ring_two_id).await?,
        ring_three: wz::equip::service::resolve_equip(state, model.ring_three_id).await?,
        ring_four: wz::equip::service::resolve_equip(state, model.ring_four_id).await?,
        pendant_one: wz::equip::service::resolve_equip(state, model.pendant_one_id).await?,
        tamed_mob: wz::equip::service::resolve_equip(state, model.tamed_mob_id).await?,
        saddle: wz::equip::service::resolve_equip(state, model.saddle_id).await?,
        medal: wz::equip::service::resolve_equip(state, model.medal_id).await?,
        belt: wz::equip::service::resolve_equip(state, model.belt_id).await?,
        pocket: wz::equip::service::resolve_equip(state, model.pocket_id).await?,
        book: wz::equip::service::resolve_equip(state, model.book_id).await?,
        pendant_two: wz::equip::service::resolve_equip(state, model.pendant_two_id).await?,
        shoulder: wz::equip::service::resolve_equip(state, model.shoulder_id).await?,
        android: wz::equip::service::resolve_equip(state, model.android_id).await?,
        emblem: wz::equip::service::resolve_equip(state, model.emblem_id).await?,
        badge: wz::equip::service::resolve_equip(state, model.badge_id).await?,
        subweapon: wz::equip::service::resolve_equip(state, model.subweapon_id).await?,
        heart: wz::equip::service::resolve_equip(state, model.heart_id).await?,
    })
}

impl RegularEquipmentSetModel {
    pub fn get_hat_id(&self) -> Result<i32, ModelError> {
        if let Some(hat_id) = self.hat_id {
            return Ok(hat_id);
        } else {
            return Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoHatId(self.char_id)),
            )));
        }
    }

    pub fn get_android_id(&self) -> Result<i32, ModelError> {
        if let Some(android_id) = self.android_id {
            Ok(android_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoAndroidId(self.char_id)),
            )))
        }
    }

    pub fn get_badge_id(&self) -> Result<i32, ModelError> {
        if let Some(badge_id) = self.badge_id {
            Ok(badge_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoBadgeId(self.char_id)),
            )))
        }
    }

    pub fn get_belt_id(&self) -> Result<i32, ModelError> {
        if let Some(belt_id) = self.belt_id {
            Ok(belt_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoBeltId(self.char_id)),
            )))
        }
    }

    pub fn get_book_id(&self) -> Result<i32, ModelError> {
        if let Some(book_id) = self.book_id {
            Ok(book_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoBookId(self.char_id)),
            )))
        }
    }

    pub fn get_bottom_id(&self) -> Result<i32, ModelError> {
        if let Some(bottom_id) = self.bottom_id {
            Ok(bottom_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoBottomId(self.char_id)),
            )))
        }
    }

    pub fn get_cape_id(&self) -> Result<i32, ModelError> {
        if let Some(cape_id) = self.cape_id {
            Ok(cape_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoCapeId(self.char_id)),
            )))
        }
    }

    pub fn get_ear_acc_id(&self) -> Result<i32, ModelError> {
        if let Some(ear_acc_id) = self.ear_acc_id {
            Ok(ear_acc_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoEarAccId(self.char_id)),
            )))
        }
    }

    pub fn get_emblem_id(&self) -> Result<i32, ModelError> {
        if let Some(emblem_id) = self.emblem_id {
            Ok(emblem_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoEmblemId(self.char_id)),
            )))
        }
    }

    pub fn get_eye_acc_id(&self) -> Result<i32, ModelError> {
        if let Some(eye_acc_id) = self.eye_acc_id {
            Ok(eye_acc_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoEyeAccId(self.char_id)),
            )))
        }
    }

    pub fn get_face_acc_id(&self) -> Result<i32, ModelError> {
        if let Some(face_acc_id) = self.face_acc_id {
            Ok(face_acc_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoFaceAccId(self.char_id)),
            )))
        }
    }

    pub fn get_gloves_id(&self) -> Result<i32, ModelError> {
        if let Some(gloves_id) = self.gloves_id {
            Ok(gloves_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoGlovesId(self.char_id)),
            )))
        }
    }

    pub fn get_heart_id(&self) -> Result<i32, ModelError> {
        if let Some(heart_id) = self.heart_id {
            Ok(heart_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoHeartId(self.char_id)),
            )))
        }
    }

    pub fn get_medal_id(&self) -> Result<i32, ModelError> {
        if let Some(medal_id) = self.medal_id {
            Ok(medal_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoMedalId(self.char_id)),
            )))
        }
    }

    pub fn get_pendant_one_id(&self) -> Result<i32, ModelError> {
        if let Some(pendant_one_id) = self.pendant_one_id {
            Ok(pendant_one_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoPendantOneId(
                    self.char_id,
                )),
            )))
        }
    }

    pub fn get_pendant_two_id(&self) -> Result<i32, ModelError> {
        if let Some(pendant_two_id) = self.pendant_two_id {
            Ok(pendant_two_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoPendantTwoId(
                    self.char_id,
                )),
            )))
        }
    }

    pub fn get_pocket_id(&self) -> Result<i32, ModelError> {
        if let Some(pocket_id) = self.pocket_id {
            Ok(pocket_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoPocketId(self.char_id)),
            )))
        }
    }

    pub fn get_ring_four_id(&self) -> Result<i32, ModelError> {
        if let Some(ring_four_id) = self.ring_four_id {
            Ok(ring_four_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoRingFourId(self.char_id)),
            )))
        }
    }

    pub fn get_ring_one_id(&self) -> Result<i32, ModelError> {
        if let Some(ring_one_id) = self.ring_one_id {
            Ok(ring_one_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoRingOneId(self.char_id)),
            )))
        }
    }

    pub fn get_ring_three_id(&self) -> Result<i32, ModelError> {
        if let Some(ring_three_id) = self.ring_three_id {
            Ok(ring_three_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoRingThreeId(self.char_id)),
            )))
        }
    }

    pub fn get_ring_two_id(&self) -> Result<i32, ModelError> {
        if let Some(ring_two_id) = self.ring_two_id {
            Ok(ring_two_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoRingTwoId(self.char_id)),
            )))
        }
    }

    pub fn get_saddle_id(&self) -> Result<i32, ModelError> {
        if let Some(saddle_id) = self.saddle_id {
            Ok(saddle_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoSaddleId(self.char_id)),
            )))
        }
    }

    pub fn get_shield_id(&self) -> Result<i32, ModelError> {
        if let Some(shield_id) = self.shield_id {
            Ok(shield_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoShieldId(self.char_id)),
            )))
        }
    }

    pub fn get_shoes_id(&self) -> Result<i32, ModelError> {
        if let Some(shoes_id) = self.shoes_id {
            Ok(shoes_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoShoesId(self.char_id)),
            )))
        }
    }

    pub fn get_shoulder_id(&self) -> Result<i32, ModelError> {
        if let Some(shoulder_id) = self.shoulder_id {
            Ok(shoulder_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoShoulderId(self.char_id)),
            )))
        }
    }

    pub fn get_subweapon_id(&self) -> Result<i32, ModelError> {
        if let Some(subweapon_id) = self.subweapon_id {
            Ok(subweapon_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoSubweaponId(self.char_id)),
            )))
        }
    }

    pub fn get_tamed_mob_id(&self) -> Result<i32, ModelError> {
        if let Some(tamed_mob_id) = self.tamed_mob_id {
            Ok(tamed_mob_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoTamedMobId(self.char_id)),
            )))
        }
    }

    pub fn get_top_id(&self) -> Result<i32, ModelError> {
        if let Some(top_id) = self.top_id {
            Ok(top_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoTopId(self.char_id)),
            )))
        }
    }

    pub fn get_weapon_id(&self) -> Result<i32, ModelError> {
        if let Some(weapon_id) = self.weapon_id {
            Ok(weapon_id)
        } else {
            Err(ModelError::from(CharacterError::from(
                EquipmentSetError::from(RegularEquipmentSetModelError::NoWeaponId(self.char_id)),
            )))
        }
    }
}
