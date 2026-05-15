/* enter_cash_shop/builder.rs
 * The purpose of this module is to build an outgoing cash shop entrance packet.
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

use crate::models::character::wrapper::Character;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_enter_cash_shop_packet(
        &mut self,
        username: String,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SetCashShop as i16;
        self.write_short(op).map_err(WriteError)?;
        // Timestamp / Session Dummy value
        self.write_long(0).map_err(WriteError)?;
        // Flag
        self.write_byte(0).map_err(WriteError)?;
        self.build_player_logged_in_meta_part_packet(char.clone())?;
        self.build_cash_shop_meta(username.clone())?;
        Ok(self)
    }

    fn build_cash_shop_meta(&mut self, username: String) -> Result<&mut Self, NetworkError> {
        // Dummy values
        // Not MTS
        self.write_byte(0).map_err(WriteError)?;
        // Account name
        self.write_str_with_length(username.clone())
            .map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        // Special cash items
        self.write_short(0).map_err(WriteError)?;
        for _ in 0..121 {
            self.write_byte(0).map_err(WriteError)?;
        }
        for _ in 0..240 {
            self.write_int(0).map_err(WriteError)?;
        }
        self.write_int(0).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }
}
