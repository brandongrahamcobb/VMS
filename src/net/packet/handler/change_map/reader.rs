use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct ChangeMapReader {
    pub died: i16,
    pub tm: i32,
    pub tn: String,
    pub wod: i16,
}

impl ChangeMapReader {
    pub fn read_change_map_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let died: i16 = pkt_reader.read_byte().map_err(ReadError)? as i16;
        let tm: i32 = pkt_reader.read_int().map_err(ReadError)?;
        let tn: String = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let skip: usize = 1;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let wheel_of_destiny_death_buff: i16 = pkt_reader.read_short().map_err(ReadError)? as i16;
        Ok(Self {
            died,
            tm,
            tn,
            wod: wheel_of_destiny_death_buff,
        })
    }
}
