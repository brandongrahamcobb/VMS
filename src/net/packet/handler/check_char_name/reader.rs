use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct CheckCharNameReader {
    pub ign: String,
}

impl CheckCharNameReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_check_char_name_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let ign = pkt_reader.read_str_with_length().map_err(ReadError)?;
        Ok(Self { ign: ign.clone() })
    }
}
