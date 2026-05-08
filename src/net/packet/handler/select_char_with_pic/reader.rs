use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct SelectCharWithPicReader {
    pub char_id: i32,
    pub mac: String,
    pub hwid: String,
    pub pic: String,
}

impl SelectCharWithPicReader {
    pub fn read_select_char_with_pic_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let pic = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let char_id = pkt_reader.read_int().map_err(ReadError)?;
        let mac = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let hwid = pkt_reader.read_str_with_length().map_err(ReadError)?;
        Ok(Self {
            char_id,
            mac,
            hwid,
            pic,
        })
    }
}
