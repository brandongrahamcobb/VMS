use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct DeleteCharReader {
    pub char_id: i32,
    pub pic: String,
}

impl DeleteCharReader {
    pub fn read_delete_char_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let pic = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let char_id = pkt_reader.read_int().map_err(ReadError)?;
        Ok(Self { char_id, pic })
    }
}
