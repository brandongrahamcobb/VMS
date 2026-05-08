use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct TosReader {
    pub confirmed: i16,
}

impl TosReader {
    pub fn read_tos_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let confirmed = pkt_reader.read_byte().map_err(ReadError)? as i16;
        Ok(Self {
            confirmed: confirmed,
        })
    }
}
