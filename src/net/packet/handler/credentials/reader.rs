use crate::inc::helpers;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct CredentialsReader {
    pub username: String,
    pub pw: String,
    pub hwid: String,
}

impl CredentialsReader {
    pub fn read_credentials_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut pkt_reader = Cursor::new(&packet.bytes);
        let _op = pkt_reader.read_short().map_err(ReadError)?;
        let username = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let pw = pkt_reader.read_str_with_length().map_err(ReadError)?;
        let skip = 6;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let hwid_bytes = 4;
        let hwid = pkt_reader.read_bytes(hwid_bytes).map_err(ReadError)?;
        let hwid = helpers::to_hex_string(&hwid);
        Ok(Self { username, pw, hwid })
    }
}
