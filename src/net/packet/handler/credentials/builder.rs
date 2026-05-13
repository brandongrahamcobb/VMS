/* credentials/builder.rs
 * The purpose of this module is to build an outgoing credentials validation packet.
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

use crate::config::settings;
use crate::models::account::wrapper::Account;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use std::time::UNIX_EPOCH;

impl Packet {
    pub fn build_credentials_handler_failed_login_packet(
        &mut self,
        status: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::AccountStatus as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(status).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_credentials_handler_successful_login_packet(
        &mut self,
        acc: Account,
    ) -> Result<&mut Self, NetworkError> {
        let pin_required = settings::get_pin_required()? as i16;
        let opcode = SendOpcode::AccountStatus as i16;
        let acc_id = acc.model.get_id()? as i32;
        let gender_wz = acc.model.gender_wz as i16;
        let account_name = acc.model.username.clone();
        let created_at: i64 = acc
            .model
            .get_created_at()?
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .try_into()?;
        self.write_short(opcode).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_int(acc_id).map_err(WriteError)?;
        self.write_byte(gender_wz).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_str_with_length(account_name)
            .map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_long(created_at).map_err(WriteError)?;
        self.write_int(1).map_err(WriteError)?;
        self.write_byte(pin_required).map_err(WriteError)?;
        self.write_byte(1).map_err(WriteError)?;
        Ok(self)
    }
}
