/* list_chars/builder.rs
 * The purpose of this module is to build an outgoing character list packet.
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

use crate::component::character::MapleCharacter;
use crate::system::packet::build::codec;
use crate::system::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_list_chars_packet(
    chars: &[MapleCharacter],
    channel_id: u8,
    char_slots: i16,
    pic_status: i16,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::CharList as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(channel_id as i16).map_err(WriteError)?;
    packet.write_byte(chars.len() as i16).map_err(WriteError)?;
    for char in chars.iter() {
        build_look_part_packet(&mut packet, char)?;
    }
    packet.write_byte(pic_status).map_err(WriteError)?;
    packet.write_int(char_slots as i32).map_err(WriteError)?;
    Ok(packet)
}

fn build_look_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
) -> Result<(), PacketBuildError> {
    codec::player::builder::build_list_char_meta_part_packet(packet, char)?;
    codec::player::builder::build_look_meta_part_packet(packet, char)?;
    packet.write_byte(0).map_err(WriteError)?;
    // Disable rank.
    packet.write_byte(0).map_err(WriteError)?;
    Ok(())
}
