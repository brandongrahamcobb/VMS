use crate::models::error::ModelError;
use crate::models::shroom::job::model::{Job, JobModel};

pub fn get_job_by_id(job_id: i16) -> Result<Job, ModelError> {
    Ok(Job {
        model: JobModel { wz_id: job_id },
    })
}

pub fn job_index_to_wz_id(index: i16) -> i32 {
    let beginner: i32 = 0;
    let cygnus: i32 = 1000;
    let aran: i32 = 2000;
    match index {
        0 => cygnus,
        1 => beginner,
        2 => aran,
        _ => -1,
    }
}
