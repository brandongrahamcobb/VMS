use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct CreateCharacterRead {
    pub ign: String,
    pub job_id: i16,
    pub face_id: i32,
    pub hair_id: i32,
    pub hair_color_id: i32,
    pub skin_id: i32,
    pub top_id: i32,
    pub bottom_id: i32,
    pub shoes_id: i32,
    pub weapon_id: i32,
    pub gender_id: i16,
}

pub fn read_create_character_packet(packet: &Packet) -> Result<CreateCharacterRead, NetworkError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let ign = pkt_reader.read_str_with_length().map_err(ReadError)?;
    let job_id = pkt_reader.read_int().map_err(ReadError)? as i16;
    let face_id = pkt_reader.read_int().map_err(ReadError)?;
    let hair_id = pkt_reader.read_int().map_err(ReadError)?;
    let hair_color_id = pkt_reader.read_int().map_err(ReadError)?;
    let skin_id = pkt_reader.read_int().map_err(ReadError)?;
    let top_id = pkt_reader
        .read_int() // Slot 5
        .map_err(ReadError)?;
    let bottom_id = pkt_reader
        .read_int() // Slot 6
        .map_err(ReadError)?;
    let shoes_id = pkt_reader
        .read_int() // Slot 7
        .map_err(ReadError)?;
    let weapon_id = pkt_reader
        .read_int() // Special
        .map_err(ReadError)?;
    let gender_id = pkt_reader.read_byte().map_err(ReadError)? as i16;
    Ok(CreateCharacterRead {
        ign,
        job_id,
        face_id,
        hair_id,
        hair_color_id,
        skin_id,
        top_id,
        bottom_id,
        shoes_id,
        weapon_id,
        gender_id,
    })
}
