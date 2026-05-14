/* player_logged_in/store.rs
 * The purpose of this module is to resolve relevant variables for player login.
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

use std::time::SystemTime;

use crate::models::character::wrapper::Character;
use crate::models::keybinding::model::{KeybindType, KeybindingModel};
use crate::models::keybinding::wrapper::Keybinding;
use crate::models::channel::wrapper::Channel;
use crate::models::map::wrapper::Map;
use crate::models::world::wrapper::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct PlayerLoggedInStore {
    pub after_players: Vec<Character>,
    pub binds: Vec<Keybinding>,
    pub channel: Channel,
    pub char: Character,
    pub map: Map,
}

impl PlayerLoggedInStore {
    pub async fn store_player_logged_in(
        state: &SharedState,
        session: Session,
        _reader: PlayerLoggedInReader,
    ) -> Result<Self, NetworkError> {
        let channel: Channel = session.get_active_channel(state).await?;
        let char: Character = session.get_active_char(state).await?;
        let map: Map = session.get_active_map(state).await?;
        let world: World = session.get_active_world(state).await?;
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
        let mut binds: Vec<Keybinding> = Vec::with_capacity(90);
        for key in 0..90 {
            binds.push(
                KeybindingModel {
                    char_id: char.model.get_id()?,
                    key: key,
                    bind_type: KeybindType::Nil as i16,
                    action: 0,
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                }
                .load()?,
            )
        }
        for bind in char.binds.clone() {
            let idx = bind.model.key as usize;
            if idx < 90 {
                binds[idx] = bind.clone();
            }
        }
        Ok(Self {
            after_players,
            binds,
            channel,
            char,
            map,
        })
    }
}
