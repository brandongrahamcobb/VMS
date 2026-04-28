use thiserror::Error;

use crate::models::account::error::AccountError;
use crate::models::channel::error::ChannelError;
use crate::models::character::error::CharacterError;
use crate::models::keybinding::error::KeybindingError;
use crate::models::world::error::WorldError;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Account model error in model layer")]
    AccountError(#[from] AccountError),

    #[error("Character model error in model layer")]
    CharacterError(#[from] CharacterError),

    #[error("Channel model error in model layer")]
    ChannelError(#[from] ChannelError),

    #[error("Keybinding model error in model layer")]
    KeybindingError(#[from] KeybindingError),

    #[error("World model error in model layer")]
    WorldError(#[from] WorldError),
}
