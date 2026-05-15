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

use std::collections::HashMap;
use std::time::SystemTime;

use crate::models::character::wrapper::Character;
use crate::models::keybinding::model::{KeybindType, KeybindingModel};
use crate::models::keybinding::wrapper::Keybinding;
use crate::models::map::wrapper::Map;
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct PlayerLoggedInStore {
    pub after_players: HashMap<i32, Character>,
    pub binds: HashMap<i32, Keybinding>,
    pub channel_id: u8,
    pub char: Character,
    pub map_wz: i32,
}

impl PlayerLoggedInStore {
    pub async fn store_player_logged_in(
        state: &SharedState,
        session: Session,
        _reader: PlayerLoggedInReader,
    ) -> Result<Self, NetworkError> {
        let channel_id: u8 = session.get_channel_id()?;
        let char_id: i32 = session.get_char_id()?;
        let char: Character = session.get_char(state).await?;
        let map: Map = session.get_map(state).await?;
        let after_players: HashMap<i32, Character> = map.chars;
        let binds: HashMap<i32, Keybinding> = (0..90)
            .map(|key| {
                Ok((
                    key,
                    KeybindingModel {
                        action: 0,
                        bind_type: KeybindType::Nil as i16,
                        char_id,
                        created_at: Some(SystemTime::now()),
                        key,
                        updated_at: SystemTime::now(),
                    }
                    .load()?,
                ))
            })
            .collect::<Result<HashMap<i32, Keybinding>, NetworkError>>()?;
        Ok(Self {
            after_players,
            binds,
            channel_id,
            char,
            map_wz: map.model.wz,
        })
    }
}
