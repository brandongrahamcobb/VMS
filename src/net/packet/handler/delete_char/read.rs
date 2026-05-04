use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::packet::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct DeleteCharRead {
    pub char_id: i32,
    pub pic: String,
}

pub fn read_delete_char_packet(packet: Packet) -> Result<DeleteCharRead, NetworkError> {
    let mut pkt_reader = Cursor::new(packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let pic = pkt_reader.read_str_with_length().map_err(ReadError)?;
    let char_id = pkt_reader.read_int().map_err(ReadError)?;
    Ok(DeleteCharRead { pic, char_id })
}
