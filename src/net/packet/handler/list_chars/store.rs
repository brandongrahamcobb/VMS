/* list_chars/store.rs
 * The purpose of this module is to resolve relevant variables for character listing.
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
use crate::models::character::wrapper::Character;
use crate::models::{account, character};
use crate::net::packet::handler::list_chars::error::ListCharsError;
use crate::net::packet::handler::list_chars::reader::ListCharsReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ListCharsStore {
    pub channel_id: u8,
    pub char_slots: i16,
    pub chars: Vec<Character>,
    pub pic_status: i16,
    pub world_id: i16,
}

pub enum PicStatus {
    Disabled = 2,
    AlreadyRegistered = 1,
    NeedsToRegister = 0,
}

impl ListCharsStore {
    pub async fn store_list_chars(
        state: &SharedState,
        session: &Session,
        reader: &ListCharsReader,
    ) -> Result<Self, ListCharsError> {
        let acc_id: i32 = session.get_acc_id()?;
        let acc: Account = account::service::get_account_by_id(state, acc_id).await?;
        let char_slots: i16 = match character::query::getters::get_char_max_by_account_and_world_id(
            state,
            acc_id,
            reader.world_id,
        )
        .await
        {
            Ok(char_max) => char_max,
            Err(_) => 8,
        };
        let mut pic_status: i16 = PicStatus::Disabled as i16;
        let use_pic = settings::get_pic_required()?;
        if let Some(_) = acc.model.pic.clone() {
            if use_pic {
                pic_status = PicStatus::AlreadyRegistered as i16;
            }
        } else {
            pic_status = PicStatus::NeedsToRegister as i16;
        };
        Ok(Self {
            channel_id: reader.channel_id,
            char_slots,
            chars: acc.chars,
            pic_status,
            world_id: reader.world_id,
        })
    }
}
