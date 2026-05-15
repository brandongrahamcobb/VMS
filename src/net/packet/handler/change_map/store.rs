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

use std::collections::HashMap;

use crate::models::channel::wrapper::Channel;
use crate::models::character::wrapper::Character;
use crate::models::error::ModelError;
use crate::models::map::error::MapError;
use crate::models::map::wrapper::Map;
use crate::models::portal::wrapper::Portal;
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_map::reader::ChangeMapReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChangeMapStore {
    pub after_map_wz: i32,
    pub after_players: HashMap<i32, Character>,
    pub before_players: HashMap<i32, Character>,
    pub channel_id: u8,
    pub char: Character,
    pub died: i16,
    pub pid: u8,
    pub wheel_of_destiny: i16,
}

impl ChangeMapStore {
    pub async fn store_change_map(
        state: &SharedState,
        session: Session,
        reader: ChangeMapReader,
    ) -> Result<Self, NetworkError> {
        let channel_id: u8 = session.get_channel_id()?;
        let channel: Channel = session.get_channel(state).await?;
        let char: Character = session.get_char(state).await?;

        let before_map: Map = session.get_map(state).await?;
        let portal: Portal = before_map.get_portal(reader.tn)?;
        let before_players: HashMap<i32, Character> = before_map.chars.clone();
        let after_map = channel
            .maps
            .get(&portal.model.tm)
            .ok_or(MapError::NotFound(portal.model.tm))
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let after_players: HashMap<i32, Character> = after_map.chars.clone();

        Ok(Self {
            after_map_wz: after_map.model.wz,
            after_players: after_players.clone(),
            before_players: before_players.clone(),
            channel_id,
            char,
            died: reader.died,
            pid: portal.model.pid,
            wheel_of_destiny: reader.wod,
        })
    }
}
