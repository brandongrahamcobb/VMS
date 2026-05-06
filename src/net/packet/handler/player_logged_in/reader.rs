use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct PlayerLoggedInReader {
    pub char_id: i32,
    pub channel_id: i8,
}

impl PlayerLoggedInReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_player_logged_in_packet(&self, packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let char_id = pkt_reader.read_int().map_err(ReadError)?;
        let channel_id = pkt_reader.read_byte().map_err(ReadError)?;
        Ok(Self {
            char_id,
            channel_id,
        })
    }
}
