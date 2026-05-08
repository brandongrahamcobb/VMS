use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct PlayerMapTransferReader;

impl PlayerMapTransferReader {
    pub fn read_player_map_transfer_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        Ok(Self)
    }
}
