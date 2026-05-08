use std::time::SystemTime;

#[derive(Clone)]
pub struct MapModel {
    pub id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct Map {
    pub model: MapModel,
}

pub struct NewMapInsert {
    pub id: i32,
}
