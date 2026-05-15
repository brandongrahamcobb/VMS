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

use std::collections::HashMap;

use crate::config::settings;
use crate::inc::helpers;
use crate::models::channel::wrapper::Channel;
use crate::models::character::wrapper::Character;
use crate::models::error::ModelError;
use crate::models::map::error::MapError;
use crate::models::map::wrapper::Map;
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChangeChannelStore {
    pub after_players: HashMap<i32, Character>,
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
    ) -> Result<Self, NetworkError> {
        let channel_id: u8 = reader.channel_id;
        let map_wz: i32 = session.get_map_wz()?;
        let channel: Channel = session.get_channel(state).await?;
        let after_map: &Map = channel
            .maps
            .get(&map_wz)
            .ok_or(MapError::NotFound(map_wz))
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let after_players: HashMap<i32, Character> = after_map.chars.clone();

        let char: Character = session.get_char(state).await?;
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        let port: i16 = channel.model.port;

        Ok(Self {
            after_players,
            channel_id,
            char,
            octets,
            port,
        })
    }
}
