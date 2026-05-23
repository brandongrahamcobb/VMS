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

use crate::packet::handler::delete_char::error::DeleteCharError;
use crate::packet::handler::delete_char::reader::DeleteCharReader;
use config::settings;
use db;
use db::pool::DbPool;
use entity;
use entity::account::wrapper::Account;
use session::model::Session;

pub struct DeleteCharStore {
    pub char_id: i32,
    pub pic_status: bool,
}

impl DeleteCharStore {
    pub async fn store_delete_char(
        pool: &DbPool,
        session: &Session,
        reader: &DeleteCharReader,
    ) -> Result<Self, DeleteCharError> {
        let acc_id: i32 = session.get_acc_id()?;
        let acc: Account = assembly::account::assemble::assemble_acc_by_id(pool, acc_id).await?;
        let use_pic = settings::get_pic_required()?;
        let mut pic_status = false;
        if use_pic {
            pic_status = entity::account::service::check_pic(acc.model.pic, reader.pic.clone());
        }
        if !pic_status {
            db::character::setters::delete_char_by_id(pool, reader.char_id).await?;
        }
        Ok(Self {
            char_id: reader.char_id,
            pic_status,
        })
    }
}
