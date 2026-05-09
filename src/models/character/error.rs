use thiserror::Error;

use crate::models::character::equipment_set::error::EquipmentSetError;

#[derive(Debug, Error)]
pub enum CharacterError {
    #[error("Requested character was not found in character model layer: {0}")]
    NotFound(i16),

    #[error("No character is selected for account in character model layer: {0}")]
    NotSelected(i32),

    #[error("Missing field in character model layer: {0}")]
    MissingField(i32),

    #[error("Equipment set error in character model layer")]
    EquipmentSetError(#[from] EquipmentSetError),

    #[error("No id found in character model layer")]
    NoId,
}
