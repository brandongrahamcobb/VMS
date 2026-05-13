/* change_map/store.rs
 * The purpose of this module is to resolve relevant variables when changing maps.
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

use crate::models::character::wrapper::Character;
use crate::models::item::inventory::wrapper::InventoryItem;
use crate::models::shroom::channel::wrapper::Channel;
use crate::models::shroom::map;
use crate::models::shroom::map::wrapper::Map;
use crate::models::shroom::portal::wrapper::Portal;
use crate::models::shroom::world::wrapper::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_map::reader::ChangeMapReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChangeMapStore {
    pub after_players: Vec<Character>,
    pub channel: Channel,
    pub char: Character,
    pub died: i16,
    pub map: Map,
    pub portal: Portal,
    pub wheel_of_destiny: i16,
}

impl ChangeMapStore {
    pub async fn store_change_map(
        state: &SharedState,
        session: Session,
        reader: ChangeMapReader,
    ) -> Result<Self, NetworkError> {
        let died: i16 = reader.died;
        let wheel_of_destiny: i16 = reader.wod;
        let char: Character = session.get_active_char(state).await?;
        let channel: Channel = session.get_active_channel(state).await?;
        let before_map: Map = session.get_active_map(state).await?;
        let world: World = session.get_active_world(state).await?;
        let before_players: Vec<Character> = Vec::<Character>::new();
        let sessions = {
            let locked_state = state.lock().await;
            locked_state.sessions.get_by_map_channel_world(
                before_map.model.wz,
                channel.model.id,
                world.model.id,
                session.id,
            )
        };
        for s in sessions {
            before_players.push(s.get_active_char(state).await?);
        }
        let portal: Portal = before_map.get_portal(reader.tn)?;
        let after_map: Map = map::service::get_map_by_id(portal.model.tm)?;
        let after_players: Vec<Character> = Vec::<Character>::new();
        let sessions = {
            let locked_state = state.lock().await;
            locked_state.sessions.get_by_map_channel_world(
                after_map.model.wz,
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
            channel,
            char,
            died,
            map: after_map.clone(),
            portal,
            wheel_of_destiny,
        })
    }
}
