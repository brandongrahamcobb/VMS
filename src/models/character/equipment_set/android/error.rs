use thiserror::Error;

#[derive(Debug, Error)]
pub enum AndroidEquipmentSetModelError {
    #[error("No hat id found in android equipment set model layer: {0}")]
    NoHatId(i32),

    #[error("No face id found in android equipment set model layer: {0}")]
    NoFaceId(i32),

    #[error("No top id found in android equipment set model layer: {0}")]
    NoTopId(i32),

    #[error("No bottom id found in android equipment set model layer: {0}")]
    NoBottomId(i32),

    #[error("No gloves id found in android equipment set model layer: {0}")]
    NoGlovesId(i32),

    #[error("No cape id found in android equipment set model layer: {0}")]
    NoCapeId(i32),
}

#[derive(Debug, Error)]
pub enum AndroidEquipmentSetError {
    #[error("No hat equip found in android equipment set layer")]
    NoHat,

    #[error("No face equip found in android equipment set layer")]
    NoFace,

    #[error("No top equip found in android equipment set layer")]
    NoTop,

    #[error("No bottom equip found in android equipment set layer")]
    NoBottom,

    #[error("No gloves equip found in android equipment set layer")]
    NoGloves,

    #[error("No cape equip found in android equipment set layer")]
    NoCape,
}
