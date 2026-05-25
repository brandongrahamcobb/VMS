/* tos/store.rs
 * The purpose of this module is to resolve relevant variables for Terms of Service.
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

use crate::tos::error::TosError;
use crate::tos::reader::TosReader;
use assembly;
use db;
use db::pool::DbPool;
use entity::account::wrapper::Account;
use session::model::Session;

pub struct TosStore {
    pub acc: Account,
    pub accepted: bool,
}

impl TosStore {
    pub async fn store_tos(
        pool: &DbPool,
        session: &Session,
        reader: &TosReader,
    ) -> Result<Self, TosError> {
        let accepted: bool = reader.confirmed == 0x01;
        let acc_id: i32 = session.get_acc_id()?;
        let acc: Account = assembly::account::assemble::assemble_acc_by_id(pool, acc_id).await?;
        if accepted {
            db::account::setters::accept_tos_by_acc_id(pool, acc.model.get_id()?).await?;
        }
        Ok(Self { acc, accepted })
    }
}
