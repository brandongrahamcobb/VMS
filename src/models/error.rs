use crate::config::error::ConfigError;
use crate::models::account::error::AccountError;
use crate::models::shroom::channel::error::ChannelError;
use crate::models::character::error::CharacterError;
use crate::models::character::keybinding::error::KeybindingError;
use crate::models::shroom::world::error::WorldError;
use crate::models::shroom::error::WzError;
use diesel;
use thiserror::Error;

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

    #[error("Config error in model layer")]
    ConfigError(#[from] ConfigError),

    #[error("Wz error in model layer")]
    WzError(#[from] WzError),

    #[error("Diesel error in model layer")]
    DieselError(#[from] diesel::result::Error),

    #[error("Map error in model layer")]
    MapError,
}
