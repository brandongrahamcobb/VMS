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

use crate::build::error::PacketBuildError;
use crate::io::error::IOError::WriteError;
use crate::model::Packet;
use crate::prelude::*;
use entity::character::wrapper::Character;
use op::send::SendOpcode;

impl Packet {
    pub fn build_enter_cash_shop_packet(
        &mut self,
        username: String,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::SetCashShop as i16;
        self.write_short(op).map_err(WriteError)?;
        // Timestamp / Session Dummy value
        self.write_long(0).map_err(WriteError)?;
        // Flag
        self.write_byte(0).map_err(WriteError)?;
        self.build_player_logged_in_meta_part_packet(char, char.model.map_wz)?;
        self.build_cash_shop_meta(username.clone())?;
        Ok(self)
    }

    fn build_cash_shop_meta(&mut self, username: String) -> Result<&mut Self, PacketBuildError> {
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
