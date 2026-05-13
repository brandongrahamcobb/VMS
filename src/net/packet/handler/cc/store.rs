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
use crate::models::character::wrapper::Character;
use crate::models::item::inventory::wrapper::InventoryItem;
use crate::models::shroom::channel;
use crate::models::shroom::channel::wrapper::Channel;
use crate::models::shroom::map::wrapper::Map;
use crate::models::shroom::world::wrapper::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChangeChannelStore {
    pub after_players: Vec<Character>,
    pub char: Character,
    pub channel: Channel,
    pub octets: [u8; 4],
}

impl ChangeChannelStore {
    pub async fn store_change_channel(
        state: &SharedState,
        session: Session,
        reader: ChangeChannelReader,
    ) -> Result<Self, NetworkError> {
        let channel: Channel =
            channel::service::get_channel_by_id(state, reader.channel_id).await?;
        let map: Map = session.get_active_map(state).await?;
        let world: World = session.get_active_world(state).await?;
        let addr = settings::get_routing_address()?;
        let octets = helpers::convert_to_ip_array(addr);
        let char = session.get_active_char(state).await?;
        let mut after_players: Vec<Character> = Vec::<Character>::new();
        let sessions = {
            let state = state.lock().await;
            state.sessions.get_by_map_channel_world(
                map.model.wz,
                channel.model.id,
                world.model.id,
                session.id,
            )
        };
        for s in sessions {
            after_players.push(s.get_active_char(state).await?);
        }
        Ok(Self {
            after_players,
            char,
            channel,
            octets,
        })
    }
}
