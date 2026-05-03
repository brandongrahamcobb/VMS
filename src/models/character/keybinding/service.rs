use crate::models::character::keybinding::model::Keybinding;

pub fn normalize_keybindings(bindings: Vec<Keybinding>, char_id: i32) -> Vec<Keybinding> {
    let mut result: Vec<Keybinding> = Vec::with_capacity(90);
    for i in 0..90 {
        result.push(Keybinding::empty(char_id, i as i16));
    }
    for bind in bindings {
        let idx = bind.key as usize;
        if idx < 90 {
            result[idx] = bind;
        }
    }
    result
}
