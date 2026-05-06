use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct ChangeChannelReader {
    pub channel_id: i8,
    pub tick: i32,
}

impl ChangeChannelReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_change_channel_packet(&self, packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let channel_id = pkt_reader.read_byte().map_err(ReadError)?;
        let tick = pkt_reader.read_int().map_err(ReadError)?;
        Ok(Self {
            channel_id: channel_id.clone(),
            tick: tick.clone(),
        })
    }
}
