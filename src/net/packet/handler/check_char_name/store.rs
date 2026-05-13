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

use crate::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::check_char_name::reader::CheckCharNameReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct CheckCharNameStore {
    pub exists: bool,
    pub ign: String,
}

impl CheckCharNameStore {
    pub async fn store_check_char_name(
        state: &SharedState,
        session: Session,
        reader: CheckCharNameReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(session);
        let exists =
            character::query::getters::get_char_model_by_name(state, reader.ign.clone())
                .await
                .is_ok();
        Ok(Self {
            exists,
            ign: reader.ign.clone(),
        })
    }
}
