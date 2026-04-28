use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeybindingError {
    #[error("Requested keybinding was not found in keybinding model layer: {0}")]
    NotFound(i16),
}
