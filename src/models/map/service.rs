use crate::models::error::ModelError;
use crate::models::map::model::{Map, MapModel, NewMapInsert};
use std::time::SystemTime;

pub fn get_map_by_id(map_id: i32) -> Result<Map, ModelError> {
    Ok(Map {
        model: MapModel {
            id: map_id,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        },
    })
}

pub fn get_map_by_job_id(job_id: i16) -> Result<Map, ModelError> {
    match job_id {
        1 => {
            let map_id = 0;
            get_map_by_id(map_id)
        }
        1000 => {
            let map_id = 130000000;
            get_map_by_id(map_id)
        }
        2000 => {
            let map_id = 140000000;
            get_map_by_id(map_id)
        }
        _ => Err(ModelError::MapError),
    }
}

impl Map {
    pub fn new(model: MapModel) -> Self {
        Self { model }
    }
}

impl NewMapInsert {
    pub fn default(id: i32) -> Self {
        Self { id }
    }
}
