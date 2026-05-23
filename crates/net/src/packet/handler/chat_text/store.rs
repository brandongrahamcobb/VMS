/* chat_text/store.rs
 * The purpose of this module is to resolve relevant variables during general chat.
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

use crate::packet::handler::chat_text::error::ChatTextError;
use crate::packet::handler::chat_text::reader::ChatTextReader;
use db::pool::DbPool;
use entity::account::wrapper::Account;
use session::model::Session;

pub struct ChatTextStore {
    pub admin: bool,
    pub char_id: i32,
    pub channel_id: u8,
    pub is_empty: bool,
    pub map_wz: i32,
    pub msg: String,
    pub show: i16,
    pub world_id: i16,
}

impl ChatTextStore {
    pub async fn store_chat_text(
        pool: &DbPool,
        session: &Session,
        reader: &ChatTextReader,
    ) -> Result<Self, ChatTextError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        let acc_id: i32 = session.get_acc_id()?;
        let acc: Account = assembly::account::assemble::assemble_acc_by_id(pool, acc_id).await?;
        let char_id = session.get_char_id()?;
        return Ok(Self {
            admin: acc.model.admin,
            channel_id,
            char_id,
            is_empty: reader.is_empty,
            map_wz,
            msg: reader.msg.clone(),
            show: reader.show,
            world_id,
        });
    }
}
