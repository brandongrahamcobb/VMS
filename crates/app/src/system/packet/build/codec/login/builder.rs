/* app/src/system/packet/build/codec/login/builder.rs
 * The purpose of this module is to build generic login packet bytes.
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

use crate::component::account::MapleAccount;
use crate::system::packet::build::error::PacketBuildError;
use config::settings;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn build_failed_login_packet(status: i16) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::AccountStatus as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(status).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_successful_login_packet(acc: &MapleAccount) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let opcode = SendOpcode::AccountStatus as i16;
    let pin_required = settings::get_pin_required()? as i16;
    let acc_id: i32 = acc.id;
    let gender_wz: i16 = acc.gender_wz;
    let account_name: String = acc.username.clone();
    let created_at: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .try_into()?; // not true
    packet.write_short(opcode).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    packet.write_short(0).map_err(WriteError)?;
    packet.write_int(acc_id).map_err(WriteError)?;
    packet.write_byte(gender_wz).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet
        .write_str_with_length(account_name)
        .map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_byte(0).map_err(WriteError)?;
    packet.write_long(0).map_err(WriteError)?;
    packet.write_long(created_at).map_err(WriteError)?;
    packet.write_int(1).map_err(WriteError)?;
    packet.write_byte(pin_required).map_err(WriteError)?;
    packet.write_byte(1).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_select_char_packet(
    char_id: i32,
    octets: [u8; 4],
    port: i16,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::ServerIp as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_short(0).map_err(WriteError)?;
    packet.write_bytes(octets.to_vec()).map_err(WriteError)?;
    packet.write_short(port).map_err(WriteError)?;
    packet.write_int(char_id).map_err(WriteError)?;
    packet.write_bytes(vec![0u8; 5]).map_err(WriteError)?;
    Ok(packet)
}
