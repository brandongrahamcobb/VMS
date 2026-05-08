use thiserror::Error;

use crate::models::character::equipment_set::android::error::{
    AndroidEquipmentSetError, AndroidEquipmentSetModelError,
};
use crate::models::character::equipment_set::cash::error::{
    CashEquipmentSetError, CashEquipmentSetModelError,
};
use crate::models::character::equipment_set::pet::error::{
    PetEquipmentSetError, PetEquipmentSetModelError,
};
use crate::models::character::equipment_set::regular::error::{
    RegularEquipmentSetError, RegularEquipmentSetModelError,
};

#[derive(Debug, Error)]
pub enum EquipmentSetError {
    #[error("Regular equipment set model error in equipment set model layer")]
    RegularEquipmentModelError(#[from] RegularEquipmentSetModelError),

    #[error("Cash equipment set model error in equipment set model layer")]
    CashEquipmentModelError(#[from] CashEquipmentSetModelError),

    #[error("Android equipment set model error in equipment set model layer")]
    AndroidEquipmentModelError(#[from] AndroidEquipmentSetModelError),

    #[error("Pet equipment set model error in equipment set model layer")]
    PetEquipmentModelError(#[from] PetEquipmentSetModelError),

    #[error("Regular equipment set error in equipment set model layer")]
    RegularEquipmentError(#[from] RegularEquipmentSetError),

    #[error("Cash equipment set error in equipment set model layer")]
    CashEquipmentError(#[from] CashEquipmentSetError),

    #[error("Android equipment set error in equipment set model layer")]
    AndroidEquipmentError(#[from] AndroidEquipmentSetError),

    #[error("Pet equipment set error in equipment set model layer")]
    PetEquipmentError(#[from] PetEquipmentSetError),
}
