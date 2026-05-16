/* cc/store.rs
 * The purpose of this module is to resolve relevant variables for changing channels.
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
use crate::models::character;
use crate::models::character::wrapper::Character;
use crate::net::packet::handler::cc::error::ChangeChannelError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChangeChannelStore {
    pub channel_id: u8,
    pub char: Character,
    pub octets: [u8; 4],
    pub port: i16,
}

impl ChangeChannelStore {
    pub async fn store_change_channel(
        state: &SharedState,
        session: Session,
        reader: ChangeChannelReader,
    ) -> Result<Self, ChangeChannelError> {
        let char_id = session.get_char_id()?;
        let char: Character = character::service::get_char_by_id(state, char_id).await?;
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = reader.channel_id;
        let port = {
            let state = state.lock().await;
            state
                .with_channel(world_id, channel_id, |channel| channel.model.port)
                .await?
        };
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        Ok(Self {
            channel_id,
            char,
            octets,
            port,
        })
    }
}
