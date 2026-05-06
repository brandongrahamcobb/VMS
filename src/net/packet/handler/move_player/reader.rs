use crate::net::error::NetworkError;
use crate::packet::io::error::IOError::ReadError;
use crate::packet::model::Packet;

const MOVEMENT_HEADER_LEN: usize = 9;

pub struct MovePlayerReader {
    pub movement_fragment: [u8],
    pub too_short: bool,
    pub empty: bool,
}

impl MovePlayerReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_move_player_packet(&self, packet: &Packet) -> Result<Self, NetworkError> {
        let mut too_short: bool = false;
        if packet.bytes.len() <= 2 + MOVEMENT_HEADER_LEN {
            too_short = true;
        }
        let mut empty: bool = false;
        let movement_fragment = &packet.bytes[(2 + MOVEMENT_HEADER_LEN)..];
        if movement_fragment.is_empty() || movement_fragment[0] == 0 {
            empty = true;
        }
        Ok(Self {
            movement_fragment: movement_fragment.clone(),
            too_short: too_short.clone(),
            empty: empty.clone(),
        })
    }
}
