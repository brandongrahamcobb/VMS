use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct TosReader {
    pub confirmed: i8,
}

impl TosReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_tos_packet(&self, packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let confirmed = pkt_reader.read_byte().map_err(ReadError)?;
        Ok(Self {
            confirmed: confirmed.clone(),
        })
    }
}
