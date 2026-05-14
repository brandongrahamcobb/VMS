/* session/model.rs
 * The purpose of this module is to provide the session model.
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

use crate::models::account::wrapper::Account;
use crate::models::channel::wrapper::Channel;
use crate::models::character::wrapper::Character;
use crate::models::item::wrapper::Item;
use crate::models::map::wrapper::Map;
use crate::models::world::wrapper::World;
use crate::models::{channel, map, world};
use crate::models::{character, item};
use crate::net::packet::model::Packet;
use crate::runtime::session::error::SessionError;
use crate::runtime::state::SharedState;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct Session {
    pub id: i32,
    pub acc: Option<Account>,
    pub tx: UnboundedSender<Packet>,
}

impl Session {
    pub fn get_acc(&self) -> Result<Account, SessionError> {
        let acc = if let Some(acc) = self.acc.clone() {
            acc
        } else {
            return Err(SessionError::NoAccount(self.id));
        };
        Ok(acc)
    }

    pub async fn get_active_char(&self, state: &SharedState) -> Result<Character, SessionError> {
        let char_id = if let Some(char_id) = self.get_acc()?.model.char_id {
            char_id
        } else {
            return Err(SessionError::NoChar(self.id));
        };
        let char: Character = character::service::get_char_by_id(state, char_id).await?;
        Ok(char)
    }

    pub async fn get_active_channel(&self, state: &SharedState) -> Result<Channel, SessionError> {
        let channel_id = if let Some(channel_id) = self.get_acc()?.model.channel_id {
            channel_id
        } else {
            return Err(SessionError::NoChannel(self.id));
        };
        let channel: Channel = channel::service::get_channel_by_id(state, channel_id).await?;
        Ok(channel)
    }

    pub async fn get_active_map(&self, state: &SharedState) -> Result<Map, SessionError> {
        let map_wz = if let Some(map_wz) = self.get_acc()?.model.map_wz {
            map_wz
        } else {
            return Err(SessionError::NoMap(self.id));
        };
        let map: Map = map::service::get_map_by_wz(state, map_wz).await?;
        Ok(map)
    }

    pub async fn get_active_world(&self, state: &SharedState) -> Result<World, SessionError> {
        let world_id = if let Some(world_id) = self.get_acc()?.model.world_id {
            world_id
        } else {
            return Err(SessionError::NoWorld(self.id));
        };
        let world: World = world::service::get_world_by_id(state, world_id).await?;
        Ok(world)
    }

    pub async fn get_active_inventory_items(
        &self,
        state: &SharedState,
    ) -> Result<Vec<Item>, SessionError> {
        let char_id = if let Some(char_id) = self.get_acc()?.model.char_id {
            char_id
        } else {
            return Err(SessionError::NoChar(self.id));
        };
        let items = item::service::get_items_by_char_id(state, char_id).await?;
        Ok(items)
    }
}
