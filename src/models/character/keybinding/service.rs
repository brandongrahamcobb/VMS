use crate::models::{
    character::keybinding::model::{Keybinding, KeybindingModel},
    error::ModelError,
};

impl KeybindingModel {
    pub fn load(&self) -> Result<Keybinding, ModelError> {
        Ok(Keybinding {
            model: self.clone(),
        })
    }
}
