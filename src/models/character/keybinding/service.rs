use std::time::SystemTime;

use crate::models::character::keybinding::model::{
    KeybindType, Keybinding, KeybindingModel, NewKeybindingInsert,
};

impl NewKeybindingInsert {
    pub fn default(char_id: i32, key: i32, bind_type: i16, action: i32) -> Self {
        Self {
            char_id,
            key,
            bind_type,
            action,
        }
    }
}

impl Keybinding {
    pub fn new(model: KeybindingModel) -> Self {
        Self { model }
    }
}

pub fn normalize_keybindings(bindings: Vec<KeybindingModel>, char_id: i32) -> Vec<KeybindingModel> {
    let mut result: Vec<KeybindingModel> = Vec::with_capacity(90);
    for i in 0..90 {
        let keybinding = KeybindingModel {
            id: 0,
            char_id: char_id,
            key: i,
            bind_type: KeybindType::Nil as i16,
            action: 0,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        result.push(keybinding);
    }
    for bind in bindings {
        let idx = bind.key as usize;
        if idx < 90 {
            result[idx] = bind.clone();
        }
    }
    result
}
