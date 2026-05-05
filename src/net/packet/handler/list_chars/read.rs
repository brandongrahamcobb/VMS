use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct ListCharsRead {
    pub channel_id: i8,
    pub world_id: i8,
}

pub fn read_list_chars_packet(packet: &Packet) -> Result<ListCharsRead, NetworkError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let skip = 1;
    pkt_reader.read_bytes(skip).map_err(ReadError)?;
    let world_id = pkt_reader.read_byte().map_err(ReadError)?;
    let channel_id = pkt_reader.read_byte().map_err(ReadError)?;
    Ok(ListCharsRead {
        channel_id,
        world_id,
    })
}
