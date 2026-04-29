use thiserror::Error;

use crate::net::packet::error::PacketError;

#[derive(Debug, Error)]
pub enum CharacterError {
    #[error("Requested character was not found in character model layer: {0}")]
    NotFound(i16),

    #[error("No character is selected for account in character model layer: {0}")]
    NotSelected(i32),

    #[error("Missing field in character model layer")]
    MissingField,

    #[error("Packet error in character model layer")]
    PacketError(#[from] PacketError),
}
