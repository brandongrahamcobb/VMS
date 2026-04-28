use thiserror::Error;

#[derive(Debug, Error)]
pub enum CharacterError {
    #[error("Requested character was not found in character layer: {0}")]
    NotFound(i16),

    #[error("No character is selected for account in character layer: {0}")]
    NotSelected(i64),
}
