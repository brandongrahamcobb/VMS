use crate::models::error::ModelError;
use crate::models::shroom::job::model::{Job, JobModel};

pub fn get_job_by_id(job_id: i16) -> Result<Job, ModelError> {
    Ok(Job {
        model: JobModel { wz_id: job_id },
    })
}
