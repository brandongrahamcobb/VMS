use std::time::SystemTime;

use crate::models::character::keybinding::model::{KeybindType, Keybinding, KeybindingModel};

impl KeybindingModel {
    pub fn new() -> Self {
        Self {
            id: -1,
            char_id: -1,
            key: -1,
            bind_type: -1,
            action: -1,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

impl Keybinding {
    pub fn new() -> Self {
        Self {
            model: KeybindingModel::new(),
        }
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
