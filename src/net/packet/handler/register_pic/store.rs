/* register_pic/store.rs
 * The purpose of this module is to resolve relevant variables for PIC registration.
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
use crate::inc::helpers;
use crate::models::account;
use crate::models::account::wrapper::Account;
use crate::net::packet::handler::register_pic::error::RegisterPicError;
use crate::net::packet::handler::register_pic::reader::RegisterPicReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct RegisterPicStore {
    pub char_id: i32,
    pub octets: [u8; 4],
    pub port: i16,
}

impl RegisterPicStore {
    pub async fn store_register_pic(
        state: &SharedState,
        session: &Session,
        reader: &RegisterPicReader,
    ) -> Result<Self, RegisterPicError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let port = {
            let state = state.lock().await;
            state
                .with_channel(world_id, channel_id, |channel| channel.model.port)
                .await?
        };
        let acc_id: i32 = session.get_acc_id()?;
        let acc: Account = account::service::get_account_by_id(state, acc_id).await?;
        acc.set_pic(state, reader.pic.clone()).await?;
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        Ok(Self {
            char_id: reader.char_id,
            octets,
            port,
        })
    }
}
