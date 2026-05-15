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

use crate::{
    models::{
        character,
        error::ModelError,
        job::{model::JobModel, wrapper::Job},
    },
    runtime::state::SharedState,
};

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

pub async fn load_job(state: &SharedState, char_id: i32) -> Result<Job, ModelError> {
    let char_model = character::query::getters::get_char_model_by_id(state, char_id).await?;
    let job_model = JobModel {
        wz: char_model.job_wz,
    };
    Ok(job_model.load()?)
}
