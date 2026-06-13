/* app/src/system/dispatch/chat_text.rs
 * The purpose of this module is to read and incoming general chat packet.
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

use net::packet::io::error::IOError::ReadError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use std::io::Cursor;

use crate::message::packet::chat_text::ReadChatTextRequestMessage;
use crate::system::packet::dispatch::error::DispatchError;

pub fn read_chat_text_packet(
    packet: &Packet,
    client_id: i32,
) -> Result<ReadChatTextRequestMessage, DispatchError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let msg = pkt_reader.read_str_with_length().map_err(ReadError)?;
    let show = pkt_reader.read_byte().map_err(ReadError)? as i16;
    let mut is_empty = false;
    if msg.is_empty() {
        is_empty = true;
    }
    Ok(ReadChatTextRequestMessage {
        client_id,
        msg,
        show,
        is_empty,
    })
}
