/* metadata/src/job/skill.rs
 * The purpose of this module is to provide metadata access to job skills.
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

use crate::job::error::JobMetadataError;
use crate::service;

pub fn get_job_skill_wzs_by_job_wz(job_wz: i16) -> Result<Vec<i32>, JobMetadataError> {
    let modified_wz: i16 = job_wz * 100;
    let mut skills: Vec<i32> = Vec::new();
    let filename: String = String::from("Skill.wz");
    let json = service::wz_to_img(modified_wz as i32, &filename)?;
    let job_wz_skill = json["skill"]
        .as_object()
        .ok_or(JobMetadataError::NoSkill(modified_wz))?;
    for (wz, _) in job_wz_skill {
        let wz: i32 = wz.parse::<i32>()?;
        skills.push(wz);
    }
    Ok(skills)
}
