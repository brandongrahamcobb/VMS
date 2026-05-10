use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct ChatTextReader {
    pub msg: String,
    pub show: i16,
    pub is_empty: bool,
}

impl ChatTextReader {
    pub fn read_chat_text_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let msg = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let show = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let mut is_empty = false;
        if msg.is_empty() {
            is_empty = true;
        }
        Ok(Self {
            msg,
            show,
            is_empty,
        })
    }
}
