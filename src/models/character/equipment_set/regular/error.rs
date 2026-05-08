use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegularEquipmentSetModelError {
    #[error("No android id found in regular equipment set model layer: {0}")]
    NoAndroidId(i32),

    #[error("No badge id found in regular equipment set model layer: {0}")]
    NoBadgeId(i32),

    #[error("No belt id found in regular equipment set model layer: {0}")]
    NoBeltId(i32),

    #[error("No book id found in regular equipment set model layer: {0}")]
    NoBookId(i32),

    #[error("No bottom id found in regular equipment set model layer: {0}")]
    NoBottomId(i32),

    #[error("No cape id found in regular equipment set model layer: {0}")]
    NoCapeId(i32),

    #[error("No ear acc id found in regular equipment set model layer: {0}")]
    NoEarAccId(i32),

    #[error("No emblem id found in regular equipment set model layer: {0}")]
    NoEmblemId(i32),

    #[error("No eye acc id found in regular equipment set model layer: {0}")]
    NoEyeAccId(i32),

    #[error("No face acc id found in regular equipment set model layer: {0}")]
    NoFaceAccId(i32),

    #[error("No gloves id found in regular equipment set model layer: {0}")]
    NoGlovesId(i32),

    #[error("No hat id found in regular equipment set model layer: {0}")]
    NoHatId(i32),

    #[error("No heart id found in regular equipment set model layer: {0}")]
    NoHeartId(i32),

    #[error("No medal id found in regular equipment set model layer: {0}")]
    NoMedalId(i32),

    #[error("No pendant one id found in regular equipment set model layer: {0}")]
    NoPendantOneId(i32),

    #[error("No pendant two id found in regular equipment set model layer: {0}")]
    NoPendantTwoId(i32),

    #[error("No pocket id found in regular equipment set model layer: {0}")]
    NoPocketId(i32),

    #[error("No ring four id found in regular equipment set model layer: {0}")]
    NoRingFourId(i32),

    #[error("No ring one id found in regular equipment set model layer: {0}")]
    NoRingOneId(i32),

    #[error("No ring three id found in regular equipment set model layer: {0}")]
    NoRingThreeId(i32),

    #[error("No ring two id found in regular equipment set model layer: {0}")]
    NoRingTwoId(i32),

    #[error("No saddle id found in regular equipment set model layer: {0}")]
    NoSaddleId(i32),

    #[error("No shield id found in regular equipment set model layer: {0}")]
    NoShieldId(i32),

    #[error("No shoes id found in regular equipment set model layer: {0}")]
    NoShoesId(i32),

    #[error("No shoulder id found in regular equipment set model layer: {0}")]
    NoShoulderId(i32),

    #[error("No subweapon id found in regular equipment set model layer: {0}")]
    NoSubweaponId(i32),

    #[error("No tamed mob id found in regular equipment set model layer: {0}")]
    NoTamedMobId(i32),

    #[error("No top id found in regular equipment set model layer: {0}")]
    NoTopId(i32),

    #[error("No weapon id found in regular equipment set model layer: {0}")]
    NoWeaponId(i32),
}

#[derive(Debug, Error)]
pub enum RegularEquipmentSetError {
    #[error("No android equip found in regular equipment set layer")]
    NoAndroid,

    #[error("No badge equip found in regular equipment set layer")]
    NoBadge,

    #[error("No belt equip found in regular equipment set layer")]
    NoBelt,

    #[error("No book equip found in regular equipment set layer")]
    NoBook,

    #[error("No bottom equip found in regular equipment set layer")]
    NoBottom,

    #[error("No cape equip found in regular equipment set layer")]
    NoCape,

    #[error("No ear acc equip found in regular equipment set layer")]
    NoEarAcc,

    #[error("No emblem equip found in regular equipment set layer")]
    NoEmblem,

    #[error("No eye acc equip found in regular equipment set layer")]
    NoEyeAcc,

    #[error("No face acc equip found in regular equipment set layer")]
    NoFaceAcc,

    #[error("No gloves equip found in regular equipment set layer")]
    NoGloves,

    #[error("No hat equip found in regular equipment set layer")]
    NoHat,

    #[error("No heart equip found in regular equipment set layer")]
    NoHeart,

    #[error("No medal equip found in regular equipment set layer")]
    NoMedal,

    #[error("No pendant one equip found in regular equipment set layer")]
    NoPendantOne,

    #[error("No pendant two equip found in regular equipment set layer")]
    NoPendantTwo,

    #[error("No pocket equip found in regular equipment set layer")]
    NoPocket,

    #[error("No ring four equip found in regular equipment set layer")]
    NoRingFour,

    #[error("No ring one equip found in regular equipment set layer")]
    NoRingOne,

    #[error("No ring three equip found in regular equipment set layer")]
    NoRingThree,

    #[error("No ring two equip found in regular equipment set layer")]
    NoRingTwo,

    #[error("No saddle equip found in regular equipment set layer")]
    NoSaddle,

    #[error("No shield equip found in regular equipment set layer")]
    NoShield,

    #[error("No shoes equip found in regular equipment set layer")]
    NoShoes,

    #[error("No shoulder equip found in regular equipment set layer")]
    NoShoulder,

    #[error("No subweapon equip found in regular equipment set layer")]
    NoSubweapon,

    #[error("No tamed mob equip found in regular equipment set layer")]
    NoTamedMob,

    #[error("No top equip found in regular equipment set layer")]
    NoTop,

    #[error("No weapon equip found in regular equipment set layer")]
    NoWeapon,
}
