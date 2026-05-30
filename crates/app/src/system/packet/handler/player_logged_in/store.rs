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

use crate::player_logged_in::error::PlayerLoggedInError;
use crate::player_logged_in::reader::PlayerLoggedInReader;
use assembly;
use db::pool::DbPool;
use entity::character::wrapper::Character;
use entity::keybinding::wrapper::Keybinding;
use std::collections::HashMap;

pub struct PlayerLoggedInStore {
    pub binds: HashMap<i32, Keybinding>,
    pub channel_id: u8,
    pub char: Character,
    pub map_wz: i32,
}

impl PlayerLoggedInStore {
    pub async fn store_player_logged_in(
        pool: &DbPool,
        reader: &PlayerLoggedInReader,
    ) -> Result<Self, PlayerLoggedInError> {
        let mut char: Character =
            assembly::character::assemble::assemble_char_by_id(pool, reader.char_id).await?;
        let map_wz = char.model.map_wz;
        let mut binds: HashMap<i32, Keybinding> =
            assembly::keybinding::assemble::assemble_keybindings_by_char_id(pool, reader.char_id)
                .await?;
        for (key, bind) in char.binds.drain() {
            binds.insert(key, bind);
        }
        Ok(Self {
            binds,
            channel_id: reader.channel_id as u8,
            char,
            map_wz,
        })
    }
}
