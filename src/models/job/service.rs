/* job/service.rs
 * The purpose of this module is to provide assisting functions and implementations for jobs.
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

use crate::metadata;
use crate::models::job::error::JobError;
use crate::models::job::model::JobWzSkill;

pub fn job_index_to_id(index: i16) -> i32 {
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

pub fn get_job_wz_skills_by_job_wz(job_wz: i16) -> Result<Vec<JobWzSkill>, JobError> {
    let modified_wz: i16 = job_wz * 100;
    let mut skills: Vec<JobWzSkill> = Vec::new();
    let filename: String = String::from("Skill.wz");
    let json = metadata::service::wz_to_img(modified_wz as i32, &filename)?;
    let job_wz_skill = json["skill"]
        .as_object()
        .ok_or(JobError::NoSkill(modified_wz))?;
    for (wz, _) in job_wz_skill {
        let wz: i32 = wz.parse::<i32>()?;
        skills.push(JobWzSkill { wz });
    }
    Ok(skills)
}
