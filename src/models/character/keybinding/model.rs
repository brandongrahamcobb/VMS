use crate::db::schema::keybindings;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Identifiable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = keybindings)]
pub struct KeybindingModel {
    pub id: i32,
    pub char_id: i32,
    pub key: i32,
    pub bind_type: i16,
    pub action: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone, Insertable, AsChangeset)]
#[diesel(table_name = keybindings)]
pub struct NewCharacterKeybindingInsert {
    pub char_id: i32,
    pub key: i32,
    pub bind_type: i16,
    pub action: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum KeybindType {
    Nil = 0,
    Skill = 1,
    Item = 2,
    Cash = 3,
    Menu = 4,
    Action = 5,
    Face = 6,
    Macro = 7,
    Text = 8,
}

#[derive(Clone)]
pub struct Keybinding {
    pub model: KeybindingModel,
}
