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

use crate::net::packet::model::Packet;
use crate::runtime::session::error::SessionError;
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
    pub fn get_acc_id(&self) -> Result<i32, SessionError> {
        self.acc_id.ok_or(SessionError::NoAccount(self.id))
    }

    pub fn get_char_id(&self) -> Result<i32, SessionError> {
        self.char_id.ok_or(SessionError::NoChar(self.id))
    }

    pub fn get_channel_id(&self) -> Result<u8, SessionError> {
        self.channel_id.ok_or(SessionError::NoChannel(self.id))
    }

    pub fn get_map_wz(&self) -> Result<i32, SessionError> {
        self.map_wz.ok_or(SessionError::NoMap(self.id))
    }

    pub fn get_world_id(&self) -> Result<i16, SessionError> {
        self.world_id.ok_or(SessionError::NoWorld(self.id))
    }
}
