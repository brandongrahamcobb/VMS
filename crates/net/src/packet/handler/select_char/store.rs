/* select_char/store.rs
 * The purpose of this module is to resolve relevant variables for no-PIC, character selection.
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

use crate::packet::handler::select_char::error::SelectCharError;
use crate::packet::handler::select_char::reader::SelectCharReader;
use config::settings;
use inc::helpers;
use session::model::Session;
use state::model::SharedState;

pub struct SelectCharStore {
    pub char_id: i32,
    pub octets: [u8; 4],
    pub port: i16,
}

impl SelectCharStore {
    pub async fn store_select_char(
        state: &SharedState,
        session: &Session,
        reader: &SelectCharReader,
    ) -> Result<Self, SelectCharError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        let port = {
            let state = state.lock().await;
            state
                .with_channel(world_id, channel_id, |channel| channel.model.port)
                .await?
        };
        Ok(Self {
            char_id: reader.char_id,
            octets,
            port,
        })
    }
}
