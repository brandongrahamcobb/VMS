/* check_char_name/store.rs
 * The purpose of this module is to resolve relevant variables checking a character name.
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

use crate::check_char_name::error::CheckCharNameError;
use crate::check_char_name::reader::CheckCharNameReader;
use db;
use db::pool::DbPool;

pub struct CheckCharNameStore {
    pub exists: bool,
    pub ign: String,
}

impl CheckCharNameStore {
    pub async fn store_check_char_name(
        pool: &DbPool,
        reader: &CheckCharNameReader,
    ) -> Result<Self, CheckCharNameError> {
        let exists: bool = db::character::getters::get_char_model_by_name(pool, reader.ign.clone())
            .await
            .is_ok();
        Ok(Self {
            exists,
            ign: reader.ign.clone(),
        })
    }
}
