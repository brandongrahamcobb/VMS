use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::packet::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct PlayerLoggedInRead {
    pub char_id: i32,
    pub channel_id: i8,
}

pub fn read_play_packet(packet: Packet) -> Result<PlayerLoggedInRead, NetworkError> {
    let mut pkt_reader = Cursor::new(packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let char_id = pkt_reader.read_int().map_err(ReadError)?;
    let channel_id = pkt_reader.read_byte().map_err(ReadError)?;
    Ok(PlayerLoggedInRead {
        char_id,
        channel_id,
    })
}
