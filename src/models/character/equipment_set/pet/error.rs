use thiserror::Error;

#[derive(Debug, Error)]
pub enum PetEquipmentSetModelError {
    #[error("No accessory one id found in pet equipment set model layer: {0}")]
    NoAccessoryOneId(i32),

    #[error("No accessory two id found in pet equipment set model layer: {0}")]
    NoAccessoryTwoId(i32),

    #[error("No accessory three id found in pet equipment set model layer: {0}")]
    NoAccessoryThreeId(i32),
}

#[derive(Debug, Error)]
pub enum PetEquipmentSetError {
    #[error("No accessory one found in pet equipment set layer")]
    NoAccessoryOne,

    #[error("No accessory two found in pet equipment set layer")]
    NoAccessoryTwo,

    #[error("No accessory three found in pet equipment set layer")]
    NoAccessoryThree,
}
