use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use std::io::Cursor;

pub struct ListWorldsReader;

impl ListWorldsReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_list_worlds(&self, packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
    }
}
