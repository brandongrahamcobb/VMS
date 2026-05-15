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
use crate::models::map::wrapper::Map;
use crate::models::world::wrapper::World;
use crate::models::{account, character};
use crate::net::packet::model::Packet;
use crate::runtime::session::error::SessionError;
use crate::runtime::state::SharedState;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct Session {
    pub id: i32,
    pub acc_id: Option<i32>,
    pub channel_id: Option<u8>,
    pub char_id: Option<i32>,
    pub map_wz: Option<i32>,
    pub world_id: Option<i16>,
    pub tx: UnboundedSender<Packet>,
}

impl Session {
    pub async fn get_acc(&self, state: &SharedState) -> Result<Account, SessionError> {
        let acc_id = self.get_acc_id()?;
        let acc: Account = account::service::get_account_by_id(state, acc_id).await?;
        Ok(acc)
    }

    pub fn get_acc_id(&self) -> Result<i32, SessionError> {
        self.acc_id.ok_or(SessionError::NoAccount(self.id))
    }

    pub async fn get_char(&self, state: &SharedState) -> Result<Character, SessionError> {
        let char_id = self.get_char_id()?;
        let char: Character = character::service::get_char_by_id(state, char_id).await?;
        Ok(char)
    }

    pub fn get_char_id(&self) -> Result<i32, SessionError> {
        self.char_id.ok_or(SessionError::NoChar(self.id))
    }

    pub async fn get_channel(&self, state: &SharedState) -> Result<Channel, SessionError> {
        self.get_world(state)
            .await?
            .channels
            .get(&self.get_channel_id()?)
            .ok_or(SessionError::NoChannel(self.id))
            .cloned()
    }

    pub fn get_channel_id(&self) -> Result<u8, SessionError> {
        self.channel_id.ok_or(SessionError::NoChannel(self.id))
    }

    pub async fn get_map(&self, state: &SharedState) -> Result<Map, SessionError> {
        self.get_channel(state)
            .await?
            .maps
            .get(&self.get_map_wz()?)
            .ok_or(SessionError::NoMap(self.id))
            .cloned()
    }

    pub fn get_map_wz(&self) -> Result<i32, SessionError> {
        self.map_wz.ok_or(SessionError::NoMap(self.id))
    }

    pub async fn get_world(&self, state: &SharedState) -> Result<World, SessionError> {
        let state = state.lock().await;
        state
            .worlds
            .read()
            .expect("poisoined")
            .get(&self.get_world_id()?)
            .ok_or(SessionError::NoWorld(self.id))
            .cloned()
    }

    pub fn get_world_id(&self) -> Result<i16, SessionError> {
        self.world_id.ok_or(SessionError::NoWorld(self.id))
    }
}
