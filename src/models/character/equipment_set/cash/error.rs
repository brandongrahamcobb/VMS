use thiserror::Error;

#[derive(Debug, Error)]
pub enum CashEquipmentSetModelError {
    #[error("No belt id found in cash equipment set model layer: {0}")]
    NoBeltId(i32),

    #[error("No bottom id found in cash equipment set model layer: {0}")]
    NoBottomId(i32),

    #[error("No cape id found in cash equipment set model layer: {0}")]
    NoCapeId(i32),

    #[error("No ear acc id found in cash equipment set model layer: {0}")]
    NoEarAccId(i32),

    #[error("No eye acc id found in cash equipment set model layer: {0}")]
    NoEyeAccId(i32),

    #[error("No face acc id found in cash equipment set model layer: {0}")]
    NoFaceAccId(i32),

    #[error("No gloves id found in cash equipment set model layer: {0}")]
    NoGlovesId(i32),

    #[error("No hair id found in cash equipment set model layer: {0}")]
    NoHairId(i32),

    #[error("No hat id found in cash equipment set model layer: {0}")]
    NoHatId(i32),

    #[error("No pendant id found in cash equipment set model layer: {0}")]
    NoPendantId(i32),

    #[error("No ring four id found in cash equipment set model layer: {0}")]
    NoRingFourId(i32),

    #[error("No ring one id found in cash equipment set model layer: {0}")]
    NoRingOneId(i32),

    #[error("No ring three id found in cash equipment set model layer: {0}")]
    NoRingThreeId(i32),

    #[error("No ring two id found in cash equipment set model layer: {0}")]
    NoRingTwoId(i32),

    #[error("No shoes id found in cash equipment set model layer: {0}")]
    NoShoesId(i32),

    #[error("No shoulder id found in cash equipment set model layer: {0}")]
    NoShoulderId(i32),

    #[error("No subweapon id found in cash equipment set model layer: {0}")]
    NoSubweaponId(i32),

    #[error("No top id found in cash equipment set model layer: {0}")]
    NoTopId(i32),

    #[error("No weapon id found in cash equipment set model layer: {0}")]
    NoWeaponId(i32),
}

#[derive(Debug, Error)]
pub enum CashEquipmentSetError {
    #[error("No belt equip found in cash equipment set layer")]
    NoBelt,

    #[error("No bottom equip found in cash equipment set layer")]
    NoBottom,

    #[error("No cape equip found in cash equipment set layer")]
    NoCape,

    #[error("No ear acc equip found in cash equipment set layer")]
    NoEarAcc,

    #[error("No eye acc equip found in cash equipment set layer")]
    NoEyeAcc,

    #[error("No face acc equip found in cash equipment set layer")]
    NoFaceAcc,

    #[error("No gloves equip found in cash equipment set layer")]
    NoGloves,

    #[error("No hair equip found in cash equipment set layer")]
    NoHair,

    #[error("No hat equip found in cash equipment set layer")]
    NoHat,

    #[error("No pendant equip found in cash equipment set layer")]
    NoPendant,

    #[error("No ring four equip found in cash equipment set layer")]
    NoRingFour,

    #[error("No ring one equip found in cash equipment set layer")]
    NoRingOne,

    #[error("No ring three equip found in cash equipment set layer")]
    NoRingThree,

    #[error("No ring two equip found in cash equipment set layer")]
    NoRingTwo,

    #[error("No shoes equip found in cash equipment set layer")]
    NoShoes,

    #[error("No shoulder equip found in cash equipment set layer")]
    NoShoulder,

    #[error("No subweapon equip found in cash equipment set layer")]
    NoSubweapon,

    #[error("No top equip found in cash equipment set layer")]
    NoTop,

    #[error("No weapon equip found in cash equipment set layer")]
    NoWeapon,
}
