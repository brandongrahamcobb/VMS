/* chat_text/builder.rs
 * The purpose of this module is to build an outgoing general chat packet.
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
use crate::models::character::wrapper::Character;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_chat_text_packet(
        &mut self,
        acc: Account,
        char: Character,
        msg: String,
        show: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ChatText as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        self.write_byte(acc.model.admin as i16)
            .map_err(WriteError)?;
        self.write_str_with_length(msg.clone())
            .map_err(WriteError)?;
        self.write_byte(show).map_err(WriteError)?;
        Ok(self)
    }
}
