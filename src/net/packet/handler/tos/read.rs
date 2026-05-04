use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::packet::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct TOSRead {
    pub confirmed: i8,
}

pub fn read_tos_packet(packet: Packet) -> Result<TOSRead, NetworkError> {
    let mut pkt_reader = Cursor::new(packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let confirmed = pkt_reader.read_byte().map_err(ReadError)?;
    Ok(TOSRead { confirmed })
}
