use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct DespawnPlayerRead {
    pub char_id: i32,
}

impl DespawnPlayerRead {
    pub fn new() -> Self {
        Self
    }

    pub fn read_despawn_player_handler_packet(
        &self,
        packet: &Packet,
    ) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let char_id = pkt_reader.read_int().map_err(ReadError)?;
        Ok(Self {
            char_id: car_id.clone(),
        })
    }
}
