use crate::net::error::NetworkError;
use crate::packet::io::error::IOError::ReadError;
use crate::packet::model::Packet;
use std::io::Cursor;

pub struct PartySearchReader;

impl PartySearchReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_party_search_packet(&self, packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        Ok(Self)
    }
}
