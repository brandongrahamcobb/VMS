use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct RegisterPicRead {
    pub char_id: i32,
    pub mac: String,
    pub hwid: String,
    pub pic: String,
}

pub fn read_register_pic_packet(packet: &Packet) -> Result<RegisterPicRead, NetworkError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let skip = 1;
    pkt_reader.read_bytes(skip).map_err(ReadError)?;
    let char_id = pkt_reader.read_int().map_err(ReadError)?;
    let mac = pkt_reader.read_str_with_length().map_err(ReadError)?;
    let hwid = pkt_reader.read_str_with_length().map_err(ReadError)?;
    let pic = pkt_reader.read_str_with_length().map_err(ReadError)?;
    Ok(RegisterPicRead {
        char_id,
        mac,
        hwid,
        pic,
    })
}
