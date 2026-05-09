use crate::models::character::equipment_set::error::EquipmentSetError;
use crate::models::character::equipment_set::regular::error::RegularEquipmentSetModelError;
use crate::models::character::equipment_set::regular::model::{
    RegularEquipmentSet, RegularEquipmentSetModel,
};
use crate::models::character::error::CharacterError;
use crate::models::error::ModelError;
use crate::models::item::equip;
use crate::runtime::state::SharedState;

impl RegularEquipmentSetModel {
    pub async fn load(&self, state: &SharedState) -> Result<RegularEquipmentSet, ModelError> {
        Ok(RegularEquipmentSet {
            model: self.clone(),
            hat: equip::service::resolve_equip(state, self.hat_id).await?,
            face_acc: equip::service::resolve_equip(state, self.face_acc_id).await?,
            eye_acc: equip::service::resolve_equip(state, self.eye_acc_id).await?,
            ear_acc: equip::service::resolve_equip(state, self.ear_acc_id).await?,
            top: equip::service::resolve_equip(state, self.top_id).await?,
            bottom: equip::service::resolve_equip(state, self.bottom_id).await?,
            shoes: equip::service::resolve_equip(state, self.shoes_id).await?,
            gloves: equip::service::resolve_equip(state, self.gloves_id).await?,
            cape: equip::service::resolve_equip(state, self.cape_id).await?,
            shield: equip::service::resolve_equip(state, self.shield_id).await?,
            weapon: equip::service::resolve_equip(state, self.weapon_id).await?,
            ring_one: equip::service::resolve_equip(state, self.ring_one_id).await?,
            ring_two: equip::service::resolve_equip(state, self.ring_two_id).await?,
            ring_three: equip::service::resolve_equip(state, self.ring_three_id).await?,
            ring_four: equip::service::resolve_equip(state, self.ring_four_id).await?,
            pendant_one: equip::service::resolve_equip(state, self.pendant_one_id).await?,
            tamed_mob: equip::service::resolve_equip(state, self.tamed_mob_id).await?,
            saddle: equip::service::resolve_equip(state, self.saddle_id).await?,
            medal: equip::service::resolve_equip(state, self.medal_id).await?,
            belt: equip::service::resolve_equip(state, self.belt_id).await?,
            pocket: equip::service::resolve_equip(state, self.pocket_id).await?,
            book: equip::service::resolve_equip(state, self.book_id).await?,
            pendant_two: equip::service::resolve_equip(state, self.pendant_two_id).await?,
            shoulder: equip::service::resolve_equip(state, self.shoulder_id).await?,
            android: equip::service::resolve_equip(state, self.android_id).await?,
            emblem: equip::service::resolve_equip(state, self.emblem_id).await?,
            badge: equip::service::resolve_equip(state, self.badge_id).await?,
            subweapon: equip::service::resolve_equip(state, self.subweapon_id).await?,
            heart: equip::service::resolve_equip(state, self.heart_id).await?,
        })
    }

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
