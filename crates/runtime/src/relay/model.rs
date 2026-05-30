/* relay/model.rs
 * The purpose of this module is to provide the shared, login and player relay models.
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

use crate::relay::types::shared::RuntimeRelay;
use action::event::TickEvent;
use net::packet::io::{read::PacketReader, write::PacketWriter};
use net::packet::model::Packet;
use state::model::SharedState;
use tokio::sync::broadcast;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Runtime<T: RuntimeRelay> {
    pub pkt_reader: PacketReader,
    pub pkt_writer: PacketWriter,
    pub state: SharedState,
    pub relay: T,
    pub rx: UnboundedReceiver<Packet>,
}

pub struct LoginRelay {
    pub session_id: i32,
}

pub struct PlayerRelay {
    pub tick_rx: Option<broadcast::Receiver<TickEvent>>,
    pub session_id: i32,
}
