/* assembly/src/job/assemble.rs
 * The purpose of this module is to assemble a job.
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

use entity::job::model::{JobModel, JobWzSkill};
use entity::job::wrapper::{Job, JobWzInfo};
use metadata;

use crate::job::error::JobAssemblyError;

pub fn assemble_job_by_wz(job_wz: i16) -> Result<Job, JobAssemblyError> {
    let job_skills: Vec<JobWzSkill> = metadata::job::skill::get_job_skills_by_job_wz(job_wz)?;
    Ok(Job {
        model: JobModel,
        info: JobWzInfo {
            wz: job_wz,
            skills: job_skills,
        },
    })
}
