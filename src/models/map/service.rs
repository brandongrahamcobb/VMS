use crate::models::{error::ModelError, map::model::MapModel};

pub fn get_map_by_id(map_id: i32) -> Result<MapModel, ModelError> {
    Ok(MapModel { id: map_id })
}

pub fn get_map_for_job(job_id: i16) -> Result<i32, NetworkError> {
    let map_id = match job_id {
        1 => 0,
        1000 => 130000000,
        2000 => 140000000,
        _ => Err(ModelError::MapError),
    };
    let map = get_map_by_id(map_id)?;
    Ok(map)
}
