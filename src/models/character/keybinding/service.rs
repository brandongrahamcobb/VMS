use crate::models::character::keybinding::model::{Keybinding, KeybindingModel};
use crate::models::error::ModelError;

impl KeybindingModel {
    pub fn load(&self) -> Result<Keybinding, ModelError> {
        Ok(Keybinding {
            model: self.clone(),
        })
    }
}
