/* delete_char/store.rs
 * The purpose of this module is to resolve relevant variables for character deletion.
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

use crate::config::settings;
use crate::models::account::wrapper::Account;
use crate::models::character;
use crate::models::character::wrapper::Character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::delete_char;
use crate::net::packet::handler::delete_char::reader::DeleteCharReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct DeleteCharStore {
    pub char: Character,
    pub pic_status: bool,
}

impl DeleteCharStore {
    pub async fn store_delete_char(
        state: &SharedState,
        session: Session,
        reader: DeleteCharReader,
    ) -> Result<Self, NetworkError> {
        let acc: Account = session.get_acc()?;
        let char: Character = character::service::get_char_by_id(state, reader.char_id).await?;
        let use_pic = settings::get_pic_required()?;
        let mut pic_status = false;
        if use_pic {
            pic_status = delete_char::service::check_pic(acc.model.clone(), reader.pic)?;
        }
        if !pic_status {
            character::query::setters::delete_character_by_id(state, reader.char_id).await?;
        }
        Ok(Self { char, pic_status })
    }
}
