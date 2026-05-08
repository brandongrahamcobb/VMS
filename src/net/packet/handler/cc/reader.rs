use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct ChangeChannelReader {
    pub channel_id: i16,
    pub tick: i32,
}

impl ChangeChannelReader {
    pub fn read_change_channel_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let channel_id = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let tick = pkt_reader.read_int().map_err(ReadError)?;
        Ok(Self { channel_id, tick })
    }
}
