/* job/model.rs
 * The purpose of this module is to provide a job model.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::models::job;
use crate::models::job::error::JobError;
use crate::models::job::wrapper::{Job, JobWzInfo};

#[derive(Clone)]
pub struct JobWzSkill {
    pub wz: i32,
}

#[derive(Clone)]
pub struct JobModel;

impl JobModel {
    pub fn load(&self, job_wz: i16) -> Result<Job, JobError> {
        let job_wz_skills: Vec<JobWzSkill> = job::service::get_job_wz_skills_by_job_wz(job_wz)?;
        Ok(Job {
            model: self.clone(),
            info: JobWzInfo {
                skills: job_wz_skills,
            },
        })
    }
}
