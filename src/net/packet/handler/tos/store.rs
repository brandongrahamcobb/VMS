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

use crate::models::account::wrapper::Account;
use crate::net::error::NetworkError;
use crate::net::packet::handler::tos::reader::TosReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct TosStore {
    pub acc: Account,
    pub accepted: bool,
}

impl TosStore {
    pub async fn store_tos(
        state: &SharedState,
        session: Session,
        reader: TosReader,
    ) -> Result<Self, NetworkError> {
        let accepted: bool = reader.confirmed == 0x01;
        let acc: Account = session.get_acc()?;
        if accepted {
            acc.accept_tos(state).await?;
        }
        Ok(Self { acc, accepted })
    }
}
