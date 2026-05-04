use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::packet::Packet;
use crate::prelude::*;
use std::io::Cursor;

pub struct ChangeKeymapRead {
    pub keys: Vec<i32>,
    pub types: Vec<i16>,
    pub actions: Vec<i32>,
}

pub fn read_change_keymap_packet(packet: Packet) -> Result<ChangeKeymapRead, NetworkError> {
    let mut pkt_reader = Cursor::new(packet.bytes);
    let _op = pkt_reader.read_short().map_err(ReadError)?;
    let _mode = pkt_reader.read_int().map_err(ReadError)?;
    let num_binds = pkt_reader.read_int().map_err(ReadError)?;
    let mut keys: Vec<i32> = Vec::new();
    let mut types: Vec<i16> = Vec::new();
    let mut actions: Vec<i32> = Vec::new();
    for _ in 0..num_binds {
        keys.push(pkt_reader.read_int().map_err(ReadError)?);
        types.push(pkt_reader.read_byte().map_err(ReadError)? as i16);
        actions.push(pkt_reader.read_int().map_err(ReadError)?);
    }
    Ok(ChangeKeymapRead {
        keys,
        types,
        actions,
    })
}
