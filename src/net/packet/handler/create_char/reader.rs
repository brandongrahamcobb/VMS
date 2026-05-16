/* create_char/reader.rs
 * The purpose of this module is to read an incoming character creation packet.
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

use crate::net::packet::handler::create_char::error::CreateCharError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;


pub struct CreateCharReader {
    pub ign: String,
    pub job_wz: i16,
    pub face_wz: i32,
    pub hair_wz: i32,
    pub hair_color_wz: i32,
    pub skin_wz: i32,
    pub top_wz: i32,
    pub bottom_wz: i32,
    pub shoes_wz: i32,
    pub weapon_wz: i32,
    pub gender_wz: i16,
}

impl CreateCharReader {
    pub fn read_create_character_packet(packet: &Packet) -> Result<Self, CreateCharError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let ign = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let job_wz = pkt_reader.read_int().map_err(ReadError)? as i16;
        let face_wz = pkt_reader.read_int().map_err(ReadError)?;
        let hair_wz = pkt_reader.read_int().map_err(ReadError)?;
        let hair_color_wz = pkt_reader.read_int().map_err(ReadError)?;
        let skin_wz = pkt_reader.read_int().map_err(ReadError)?;
        let top_wz = pkt_reader
            .read_int() // Slot 5
            .map_err(ReadError)?;
        let bottom_wz = pkt_reader
            .read_int() // Slot 6
            .map_err(ReadError)?;
        let shoes_wz = pkt_reader
            .read_int() // Slot 7
            .map_err(ReadError)?;
        let weapon_wz = pkt_reader
            .read_int() // Special
            .map_err(ReadError)?;
        let gender_wz = pkt_reader.read_byte().map_err(ReadError)? as i16;
        Ok(Self {
            ign,
            job_wz,
            face_wz,
            hair_wz,
            hair_color_wz,
            skin_wz,
            top_wz,
            bottom_wz,
            shoes_wz,
            weapon_wz,
            gender_wz,
        })
    }
}
