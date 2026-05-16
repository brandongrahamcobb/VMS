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

use crate::models::character;
use crate::models::character::wrapper::Character;
use crate::models::portal::wrapper::Portal;
use crate::net::packet::handler::change_map::error::ChangeMapError;
use crate::net::packet::handler::change_map::reader::ChangeMapReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

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
    ) -> Result<Self, ChangeMapError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        let before_map = {
            let state = state.lock().await;
            state.get_map(world_id, channel_id, map_wz).await?
        };
        let before_players: HashMap<i32, Character> = before_map.chars.clone();
        let portal: Portal = before_map.get_portal(reader.tn)?;
        let after_map = {
            let state = state.lock().await;
            state.get_map(world_id, channel_id, portal.model.tm).await?
        };
        let after_players: HashMap<i32, Character> = after_map.chars.clone();

        let char_id = session.get_char_id()?;
        let char: Character = character::service::get_char_by_id(state, char_id).await?;

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
