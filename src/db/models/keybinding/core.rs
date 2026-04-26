use crate::db::schema::keybindings;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = keybindings)]
pub struct Keybinding {
    pub id: i32,
    pub character_id: i32,
    pub key: i16,
    pub bind_type: i16,
    pub action: i16,
}

#[derive(Clone, Insertable, AsChangeset)]
#[diesel(table_name = keybindings)]
pub struct NewKeybinding {
    pub character_id: i32,
    pub key: i16,
    pub bind_type: i16,
    pub action: i16,
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
